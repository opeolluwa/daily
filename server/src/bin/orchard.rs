use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::{DefaultBodyLimit, State},
    http::{header, HeaderName, HeaderValue, Method, StatusCode},
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use axum_governor::{
    BodyPreset, GovernorConfigBuilder, GovernorLayer, Quota, extractor::PeerIp, nz,
};
use dotenv::dotenv;
use lunar::{data_engine, error::LunarError};
use orchard_lib::{
    config::{AppConfig, Environment},
    errors::app_error::AppError,
    middlewares::csrf::CsrfLayer,
    routes::router::load_routes,
    shutdown::shutdown_signal,
    states::GraphQlState,
};
use seaography::async_graphql;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    timeout::TimeoutLayer,
    trace::{self, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use orchard_migration::{Migrator, MigratorTrait};

#[axum::debug_handler]
async fn graphql_playground(
    State(GraphQlState { endpoint, .. }): State<GraphQlState>,
) -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(&endpoint)))
}

#[axum::debug_handler]
async fn graphql_handler(
    State(GraphQlState { schema, .. }): State<GraphQlState>,
    headers: axum::http::HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner().data(headers);
    let response = schema.execute(req).await;
    if response.is_err() {
        tracing::warn!(errors = ?response.errors, "GraphQL errors in response");
    }
    response.into()
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let app_config = AppConfig::from_env()?;

    let signing_key = std::env::var("JWT_SIGNING_KEY")
        .map_err(|_| AppError::StartupError("JWT_SIGNING_KEY is not set".to_string()))?;
    if signing_key.trim().len() < 32 {
        return Err(AppError::StartupError(
            "JWT_SIGNING_KEY must be at least 32 characters".to_string(),
        ));
    }

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

// rate limit
    let requests_per_second =
        NonZeroU32::new(app_config.rate_limit_rps).unwrap_or(nz!(20u32));
    let burst = NonZeroU32::new(app_config.rate_limit_burst).unwrap_or(nz!(40u32));
    let requests_per_minute =
        NonZeroU32::new(app_config.rate_limit_rpm).unwrap_or(nz!(300u32));

    let governor = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .quota_default(Quota::requests_per_second(requests_per_second).burst(burst))
        .stack(
            "per-minute",
            PeerIp::default(),
            Quota::requests_per_minute(requests_per_minute),
        )
        .whitelist_paths(["/health"])
        .whitelist_methods([Method::OPTIONS])
        .max_keys(app_config.rate_limit_max_keys.max(1))
        .redact_keys(true)
        .body_preset(BodyPreset::Json)
        .finish()
        .map_err(|err| {
            tracing::warn!("failed to startup due to {} ", err.to_string());
            AppError::StartupError("internal server error".to_string())
        })?;

//CORS
    let cors = if app_config.environment == Environment::Production {
        CorsLayer::new()
            .allow_origin(app_config.allowed_origins.clone())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                HeaderName::from_static("x-device-id"),
            ])
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    };

//db
    let lunar = data_engine::DataEngine::new(&app_config.database_url).await?;

    lunar
        .run_migrations()
        .await
        .map_err(|e| LunarError::DbConnectError(e.to_string()))?;

    let db = lunar.connection().to_owned();
    let db_conn = Arc::new(db.clone());

    Migrator::up(&db, None)
        .await
        .map_err(|e| LunarError::DbConnectError(e.to_string()))?;

    let schema = orchard_lib::query_root::schema(db, Some(100), app_config.complexity_limit)
        .map_err(|err| AppError::GraphQLError(err.to_string()))?;

    let graphql_state = GraphQlState {
        schema,
        endpoint: app_config.graphql_endpoint.clone(),
    };

    let http_routes = load_routes(&db_conn);

    let graphql_router = Router::new()
        .route(
            &app_config.graphql_endpoint,
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(graphql_state);


    let is_production = app_config.environment.is_production();

    let app = Router::new()
        .merge(graphql_router)
        .merge(http_routes)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            app_config.body_limit_mb * 1024 * 1024,
        ))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(app_config.timeout_secs),
        ))
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(SetSensitiveHeadersLayer::new([
            header::AUTHORIZATION,
            header::COOKIE,
            header::SET_COOKIE,
        ]))
        .layer(CsrfLayer::new(
            app_config.allowed_origin_strings.clone(),
            is_production,
        ))
        .layer(GovernorLayer::new(governor))
        .layer(cors)
        .layer({
            axum::middleware::map_response(move |mut response: axum::response::Response| async move {
                let headers = response.headers_mut();
                headers.insert(
                    HeaderName::from_static("x-content-type-options"),
                    HeaderValue::from_static("nosniff"),
                );
                headers.insert(
                    HeaderName::from_static("x-frame-options"),
                    HeaderValue::from_static("DENY"),
                );
                headers.insert(
                    HeaderName::from_static("referrer-policy"),
                    HeaderValue::from_static("no-referrer"),
                );
                headers.insert(
                    HeaderName::from_static("permissions-policy"),
                    HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
                );
                headers.insert(
                    HeaderName::from_static("cross-origin-opener-policy"),
                    HeaderValue::from_static("same-origin"),
                );
                headers.insert(
                    HeaderName::from_static("cross-origin-resource-policy"),
                    HeaderValue::from_static("same-origin"),
                );
                headers.insert(
                    HeaderName::from_static("content-security-policy"),
                    HeaderValue::from_static(
                        "default-src 'none'; frame-ancestors 'none'; base-uri 'none'; form-action 'none'",
                    ),
                );
                if is_production {
                    headers.insert(
                        HeaderName::from_static("strict-transport-security"),
                        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
                    );
                }
                response
            })
        });

    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, app_config.port));
    tracing::info!(
        "Visit GraphQL Playground at http://{}{}",
        ip_address,
        app_config.graphql_endpoint
    );
    tracing::info!("Service health check at http://{}/health", ip_address,);
    axum::serve(
        TcpListener::bind(ip_address)
            .await
            .map_err(|e| AppError::InternalError(format!("failed to bind to {ip_address}: {e}")))?,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .map_err(|err| AppError::InternalError(err.to_string()))?;

    Ok(())
}
