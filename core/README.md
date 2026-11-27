# Core

A Rust data-access layer built on SeaORM. It is shared natively by the Tauri
desktop app and the Rust server, and exports TypeScript model/DTO types via
`ts-rs` for the desktop frontend.

There is no browser/WASM build: desktop data access goes through Tauri IPC into
these repositories against a local SQLite database.

## Project layout

```
core/
├── src/
│   ├── entities/            # SeaORM models (also generate bindings/*.ts)
│   ├── adapters/            # camelCase create/update DTOs + RequestMeta
│   ├── repositories/        # async-trait repos + native constructors
│   ├── data_engine.rs       # native connect + migrations
│   └── error.rs             # KernelError
├── bindings/                # generated TS types (ts-rs)
├── index.ts                 # re-exports the bindings (auto-generated)
└── migration/               # native-only migration crate
```

## Building

```bash
cargo check
cargo build
cargo test
```

From the repo root, `just build-core` runs the tests and regenerates the
TypeScript bindings.

## Repository API

Repositories are workspace-scoped. Native callers construct them with
`RepositoryName::new(Arc<DatabaseConnection>)`; the prelude re-exports the
`*RepositoryExt` traits that define the constructors and methods.

### WorkspaceRepository

| Method | Returns |
| --- | --- |
| `create_workspace(payload)` | `Workspaces` |
| `get_workspace_by_id(identifier)` | `Workspaces` |
| `list_workspaces()` | `Workspaces[]` |
| `update_workspace(identifier, payload)` | `Workspaces` |
| `delete_workspace(identifier, meta)` | `void` |
| `verify_workspace_password(identifier, password)` | `boolean` |
| `exists(identifier)` | `boolean` |

### Notes / Bookmarks / Snippets / Reminders

Common methods (payload type varies by repository):

| Method | Returns |
| --- | --- |
| `create(payload, meta)` | `Model` |
| `find_by_id(identifier, meta)` | `Model \| null` |
| `find_all(meta)` | `Model[]` |
| `recently_added(meta)` | `Model[]` (notes/bookmarks/snippets) |
| `update(identifier, payload, meta)` | `Model` |
| `delete(identifier, meta)` | `void` |

`BookmarkRepository` additionally has `find_by_tag(tag, meta)` and `exists(identifier)`.

### TodoRepository

| Method | Returns |
| --- | --- |
| `create_todo(payload, meta)` | `Todo` |
| `find_by_id(identifier, meta)` | `Todo \| null` |
| `find_all(meta)` | `Todo[]` |
| `update(identifier, payload, meta)` | `Todo` |
| `delete(identifier, meta)` | `void` |
| `change_priority(identifier, priority, meta)` | `Todo` |
| `update_due_date(identifier, dueDate, meta)` | `Todo` |
| `mark_done(identifier, done, meta)` | `Todo` |

### NotificationRepository

`create`, `find_by_id`, `find_all`, `find_by_type(notificationType, meta)`,
`mark_as_read`, `delete`.

### RecycleBinRepository

`store`, `find_all`, `find_by_id`, `find_by_item_type(itemType, meta)`, `purge`,
`purge_all`.

### UserPreferences / WorkspacePreferences

`create`, `get_by_identifier(identifier)` / `get(meta)`, `update(identifier, payload[, meta])`.

### Workspace transfer / duplicate

Record repositories expose `transfer_record`, `duplicate_record`, and
`record_exists_in_workspace`.

## Payloads

Adapter DTOs are camelCase. `RequestMeta` is
`{ workspaceIdentifier: string }`; UUIDs are strings.

| DTO | Fields |
| --- | --- |
| `CreateWorkspace` | `name`, `description` |
| `UpdateWorkspace` | `name?`, `description?`, `isDefault?`, `isHidden?`, `isSecured?`, `password?` |
| `CreateNote` | `title`, `content`, `categories?: string[]` |
| `UpdateNote` | `title?`, `content?`, `categories?` |
| `CreateBookmark` | `title`, `url`, `tag` |
| `UpdateBookmark` | `title?`, `url?`, `tag?` |
| `CreateSnippet` | `title?`, `language?`, `code`, `description?`, `isPinned`, `createdAt`, `updatedAt` |
| `UpdateSnippet` | `title?`, `language?`, `code?`, `description?`, `isPinned?` |
| `CreateTodo` | `title`, `description?`, `dueDate?`, `priority` |
| `UpdateTodo` | `title?`, `description?` |
| `CreateReminder` | `title`, `description?`, `recurring`, `recurrenceRule?`, `alarmSound?`, `remindAt` |
| `UpdateReminder` | `title?`, `description?`, `recurring?`, `recurrenceRule?`, `alarmSound?`, `remindAt?` |
| `CreateNotification` | `title`, `body`, `notificationType`, `isRead` |
| `CreateRecycleBinEntry` | `itemId`, `itemType`, `payload` (JSON string) |
| `CreateUserPreferences` | `masterFirstName`, `masterLastName`, `masterEmail` |
| `UpdateUserPreferences` | `masterFirstName?`, `masterLastName?`, `masterEmail?` |

Timestamps are RFC3339 / date strings; enums serialize as PascalCase strings
matching `bindings/sea_orm_active_enums.ts`.

## Native usage

```rust
use std::sync::Arc;
use kernel::data_engine::DataEngine;
use kernel::repositories::{prelude::*, workspace::WorkspaceRepository};

#[tokio::main]
async fn main() -> Result<(), kernel::error::KernelError> {
    let engine = DataEngine::new("postgres://user:pass@localhost:6543/orchard").await?;
    engine.run_migrations().await?;

    let conn = engine.connection().clone();
    let workspaces = WorkspaceRepository::new(Arc::new(conn));
    let workspace = workspaces
        .create_workspace(kernel::adapters::workspace::CreateWorkspace {
            name: "Work".into(),
            description: String::new(),
        })
        .await?;

    Ok(())
}
```

The desktop app uses a SQLite `DatabaseConnection` from `DataEngine` in the
Tauri setup (`console/src-tauri/src/lib.rs`); the server uses Postgres.

## TypeScript types

`index.ts` + `bindings/` are pure TypeScript types. The desktop frontend
depends on the `core/` package root for `Model`, `Create*`, `RequestMeta`, and
enum types. `bindings/` is regenerated from `#[ts(export)]` entities;
`index.ts` is generated by `scripts/fix-ts-exports.mjs` — do not edit by hand.
