set dotenv-required := false
set dotenv-load
set dotenv-path := ".env"
set export

DOCKER_CMD := "docker compose -f docker-compose.yaml"
POSTGRES_URL := "postgres://almond:almond@localhost:5433/almond"
MYSQL_URL := "mysql://almond:almond@localhost:3307/almond"
SQLITE_URL := "sqlite://./almonds-dev.db?mode=rwc"
DATABASE_URL := "postgres://orchard:orchard@localhost:6543/orchard"

alias w := watch
alias b := build
alias cfg := configure
alias test-tauri := test-console-tauri

configure:
    just install-dependencies
    just install-frontend-dependencies
    chmod +x scripts/release.sh

watch target:
    just watch-{{ target }}

build target:
    just build-{{ target }}

lint target:
    #!/usr/bin/env bash
    if [ "{{ target }}" = "all" ]; then
    	just lint-console
    	just lint-server
    	just lint-console-tauri
    else
    	just lint-{{ target }}
    fi

test target:
    #!/usr/bin/env bash
    if [ "{{ target }}" = "all" ]; then
    	just test-console
    	just test-core
    	just test-server
    	just test-console-tauri
    else
    	just test-{{ target }}
    fi

release target:
    @just release-{{ target }}

clean target:
    #!/usr/bin/env sh
    echo "Cleaning {{ target }} build assets"
    if [ "{{ target }}" = "console" ]; then
    	just clean-console
    elif [ "{{ target }}" = "server" ]; then
    	just clean-server
    elif [ "{{ target }}" = "all" ]; then
    	just clean-console
    	just clean-server
    else
    	echo "Invalid target '{{ target }}'. Use one of: console, server, all"
    fi

sync:
    sh scripts/sync.sh

# --- Core ---

[working-directory('core')]
watch-core:
    cargo watch -qcx check

# Start postgres and mysql containers (docker/docker-compose.yml)
[working-directory('core')]
db-up:
    docker compose -f docker/docker-compose.yml up -d

[working-directory('core')]
@migrate-add target:
    @sea-orm-cli migrate generate "{{ target }}"

[working-directory('core')]
@migrate-init:
    @sea-orm-cli migrate init

# Run pending migrations against postgres (docker)
[working-directory('core')]
@migrate-up-postgres:
    sea-orm-cli migrate up --database-url {{ POSTGRES_URL }}

[working-directory('server')]
@migrate-up-server:
    sea-orm-cli migrate up --database-url {{ DATABASE_URL }}

[working-directory('core')]
@migrate-up:
    just migrate-up-postgres

# Pull schema and generate entities for PostgreSQL using docker defaults (feature: postgres)
[working-directory('core')]
@db-pull-postgres:
    sea-orm-cli generate entity \
    	--database-url {{ POSTGRES_URL }} \
    	--with-serde both \
    	--model-extra-attributes 'serde(rename_all="camelCase")' \
    	--model-extra-attributes 'kernel_macros::ts_rs_export_sea_orm_entity_name' \
    	--enum-extra-derives 'ts_rs::TS' \
    	--enum-extra-attributes 'ts(export)' \
    	--enum-extra-attributes 'serde(rename_all="snake_case")' \
    	--er-diagram \
    	-o src/entities \
    	--seaography
    node ../scripts/fix-ts-exports.mjs src/entities

# Generate entities for all backends.
# Starts docker, runs migrations, then generates entities.
# just db-pull
[working-directory('core')]
db-pull:
    just db-up
    @echo "Waiting for PostgreSQL to be ready..."
    just migrate-up
    just db-pull-postgres
    # just db-pull-server

[working-directory('core')]
test-core:
    cargo test

db-pull-server:
    just migrate-up-server
    just generate-server-entities

# --- WASM build & PGlite migration export ---

# Export migration SQL to generated/pglite/migrations, then build the WASM bundle.
#   just build-core          # dev profile (matches pkg/ checked into the repo)
# just build-core release  # release profile
[working-directory('core')]
build-core profile='dev':
    cargo run --manifest-path migration/Cargo.toml --bin migration-export -- generated/pglite/migrations postgres
    wasm-pack build --target web --{{ profile }}
    cargo test
    node ../scripts/fix-ts-exports.mjs src/entities

# Export PGlite migration SQL only (no WASM build).
# This is what console/scripts/sync-core-schema.mjs runs if the generated
# files are missing.
[working-directory('core')]
schema-export:
    cargo run --manifest-path migration/Cargo.toml --bin migration-export -- generated/pglite/migrations postgres

# --- Console ---

[working-directory('console')]
watch-frontend:
    npm run dev

[working-directory('console')]
watch-tauri:
    npm run tauri dev

[working-directory('console')]
watch-mobile:
    ./src-tauri/gen/android/gradlew --stop
    npm run tauri android dev

[working-directory('console')]
watch-mobile-device HOST:
    TAURI_DEV_HOST={{ HOST }} npm run tauri android dev

[working-directory('console')]
watch-ios:
    npm run tauri ios dev

[working-directory('console')]
watch-ios-device HOST:
    TAURI_DEV_HOST={{ HOST }} npm run tauri ios dev

[working-directory('console')]
watch-console:
    npm run dev & npm run tauri dev

[working-directory('console')]
build-console:
    npm run tauri build

[working-directory('console')]
build-mobile:
    ./src-tauri/gen/android/gradlew --stop
    npm run tauri android build  -- --apk

[working-directory('console')]
install-frontend-dependencies:
    npm install

[working-directory('console')]
test-console:
    echo "TBD"

[working-directory('console/src-tauri')]
test-console-tauri:
    echo "TBD"

# --- Server ---

# Build the production Docker image for the app server.
# Usage: just build app  (via the root `build` alias)
#        just build-app  (direct)
# just build-app my-registry.io/orchard  (custom tag prefix)
build-app tag="orchard":
    DOCKER_BUILDKIT=1 docker build \
    	--file docker/prod/Dockerfile.app \
    	--tag {{ tag }}:latest \
    	--tag {{ tag }}:$(git rev-parse --short HEAD) \
    	.

[working-directory('server')]
server-init url:
    seaography-cli -e ./src/entities  --output-dir . --database-url={{ url }} --framework axum orchard

@server-logs:
    {{ DOCKER_CMD }} logs -f --tail='30' app

@watch-server:
    {{ DOCKER_CMD }} up -d
    @just server-logs

[working-directory('server')]
test-server:
    echo "TBD"

[working-directory('server')]
@migrate-server-add target:
    @sea-orm-cli migrate generate "{{ target }}"

[working-directory('server')]
@migrate-server-init:
    @sea-orm-cli migrate init

# Pull schema and generate entities + seaography bindings for PostgreSQL
[working-directory('server')]
@generate-server-entities:
    sea-orm-cli generate entity \
    	--database-url {{ DATABASE_URL }} \
    	--with-serde both \
    	--model-extra-attributes 'serde(rename_all="camelCase")' \
    	-o src/entities

# --- Lint ---

[working-directory('server')]
lint-server:
    sh ../scripts/format.sh

[working-directory('console/src-tauri')]
lint-console-tauri:
    sh ../../scripts/format.sh

[working-directory('console')]
lint-console:
    npm run format
    npm run lint:fix

# --- Clean ---

[working-directory('console')]
clean-console:
    rm -rf node_modules dist .nuxt
    cargo clean --manifest-path src-tauri/Cargo.toml

[working-directory('server')]
clean-server:
    cargo clean

# --- Dependencies ---

install-dependencies:
    cargo install cargo-watch
    cargo install cargo-sort
    cargo install cargo-group-imports
    cargo install sea-orm-cli@^2.0.0-rc
    cargo install seaography-cli@^2.0.0-rc
    cargo install wasm-pack

# --- Release ---

release-patch:
    ./scripts/release.sh patch

release-minor:
    ./scripts/release.sh minor

release-major:
    ./scripts/release.sh major

sort-env:
    node scripts/sort-env.mjs

env-diff:
    node scripts/env-diff.mjs
