use intelligence::{
    agent::DEFAULT_SYSTEM_PROMPT, AppContext, ChatMessage, ContextItem, IntelligenceClient,
    ProviderConfig, StreamEvent,
};
use tauri::ipc::Channel;
use tauri::State;
use tokio::sync::mpsc;

use crate::errors::AppError;
use crate::state::app::AppState;

#[tauri::command]
pub async fn intelligence_configure(
    state: State<'_, AppState>,
    config: ProviderConfig,
) -> Result<(), AppError> {
    let client = IntelligenceClient::new(config)?;
    *state.intelligence_client.write().await = Some(client);
    Ok(())
}

#[tauri::command]
pub async fn intelligence_health_check(state: State<'_, AppState>) -> Result<bool, AppError> {
    let guard = state.intelligence_client.read().await;
    let client = guard.as_ref().ok_or_else(|| {
        AppError::Intelligence(intelligence::IntelligenceError::Unavailable(
            "No provider configured".into(),
        ))
    })?;
    client.health_check().await.map_err(Into::into)
}

#[tauri::command]
pub async fn intelligence_list_models(state: State<'_, AppState>) -> Result<Vec<String>, AppError> {
    let guard = state.intelligence_client.read().await;
    let client = guard.as_ref().ok_or_else(|| {
        AppError::Intelligence(intelligence::IntelligenceError::Unavailable(
            "No provider configured".into(),
        ))
    })?;
    client.list_models().await.map_err(Into::into)
}

#[tauri::command]
pub async fn intelligence_chat(
    state: State<'_, AppState>,
    messages: Vec<ChatMessage>,
) -> Result<intelligence::CompletionResponse, AppError> {
    let guard = state.intelligence_client.read().await;
    let client = guard.as_ref().ok_or_else(|| {
        AppError::Intelligence(intelligence::IntelligenceError::Unavailable(
            "No provider configured".into(),
        ))
    })?;
    client.complete(&messages).await.map_err(Into::into)
}

#[tauri::command]
pub async fn intelligence_chat_stream(
    state: State<'_, AppState>,
    messages: Vec<ChatMessage>,
    on_event: Channel<StreamEvent>,
) -> Result<(), AppError> {
    let guard = state.intelligence_client.read().await;
    let client = guard.as_ref().ok_or_else(|| {
        AppError::Intelligence(intelligence::IntelligenceError::Unavailable(
            "No provider configured".into(),
        ))
    })?;

    let (tx, mut rx) = mpsc::channel::<StreamEvent>(64);

    let stream_future = client.stream(&messages, tx);

    let forward_handle = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            if on_event.send(event).is_err() {
                break;
            }
        }
    });

    let result = stream_future.await;
    let _ = forward_handle.await;

    result.map_err(Into::into)
}

#[tauri::command]
pub async fn intelligence_agent_chat_stream(
    state: State<'_, AppState>,
    user_message: String,
    history: Vec<ChatMessage>,
    include_context: bool,
    on_event: Channel<StreamEvent>,
) -> Result<(), AppError> {
    let guard = state.intelligence_client.read().await;
    let client = guard.as_ref().ok_or_else(|| {
        AppError::Intelligence(intelligence::IntelligenceError::Unavailable(
            "No provider configured".into(),
        ))
    })?;

    let context = if include_context {
        build_app_context(&state).await?
    } else {
        AppContext::default()
    };

    let context_section = context.to_system_prompt_section();
    let system_prompt = if context_section.is_empty() {
        DEFAULT_SYSTEM_PROMPT.to_string()
    } else {
        format!("{DEFAULT_SYSTEM_PROMPT}\n\n---\n\nUser's current data:\n\n{context_section}")
    };

    let mut messages = vec![ChatMessage::system(system_prompt)];
    messages.extend(history);
    messages.push(ChatMessage::user(user_message));

    let (tx, mut rx) = mpsc::channel::<StreamEvent>(64);

    let stream_future = client.stream(&messages, tx);

    let forward_handle = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            if on_event.send(event).is_err() {
                break;
            }
        }
    });

    let result = stream_future.await;
    let _ = forward_handle.await;

    result.map_err(Into::into)
}

#[tauri::command]
pub async fn intelligence_get_config(
    state: State<'_, AppState>,
) -> Result<Option<ProviderConfig>, AppError> {
    let guard = state.intelligence_client.read().await;
    Ok(guard.as_ref().map(|c| c.config().clone()))
}

async fn build_app_context(state: &AppState) -> Result<AppContext, AppError> {
    use kernel::repositories::bookmarks::BookmarkRepositoryExt;
    use kernel::repositories::notes::NotesRepositoryExt;
    use kernel::repositories::prelude::*;

    let notes = state
        .notes_repository
        .recently_added(&None)
        .await
        .unwrap_or_default();

    let note_items: Vec<ContextItem> = notes
        .into_iter()
        .take(10)
        .map(|n| ContextItem {
            title: n.title,
            summary: n.content.chars().take(200).collect(),
            identifier: Some(n.identifier),
        })
        .collect();

    let tasks = state
        .task_repository
        .find_all(&None)
        .await
        .unwrap_or_default();

    let task_items: Vec<ContextItem> = tasks
        .into_iter()
        .filter(|t| !t.done)
        .take(20)
        .map(|t| ContextItem {
            title: t.title.clone(),
            summary: t.description.clone().unwrap_or_default(),
            identifier: Some(t.identifier.clone()),
        })
        .collect();

    let bookmarks = state
        .bookmark_repository
        .recently_added(&None)
        .await
        .unwrap_or_default();

    let bookmark_items: Vec<ContextItem> = bookmarks
        .into_iter()
        .take(10)
        .map(|b| ContextItem {
            title: b.title,
            summary: b.url,
            identifier: Some(b.identifier),
        })
        .collect();

    Ok(AppContext {
        notes: note_items,
        tasks: task_items,
        bookmarks: bookmark_items,
        reminders: Vec::new(),
        extra: None,
    })
}
