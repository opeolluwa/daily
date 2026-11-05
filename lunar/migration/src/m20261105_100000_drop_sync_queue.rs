use sea_orm_migration::{prelude::*, sea_orm::DbBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();

        if db_backend == DbBackend::Sqlite {
            db_connection
                .execute_unprepared(
                    r#"
DROP TRIGGER IF EXISTS bookmark_sync_insert;
DROP TRIGGER IF EXISTS bookmark_sync_update;
DROP TRIGGER IF EXISTS bookmark_sync_delete;
DROP TRIGGER IF EXISTS notes_sync_insert;
DROP TRIGGER IF EXISTS notes_sync_update;
DROP TRIGGER IF EXISTS notes_sync_delete;
DROP TRIGGER IF EXISTS recycle_bin_sync_insert;
DROP TRIGGER IF EXISTS recycle_bin_sync_update;
DROP TRIGGER IF EXISTS recycle_bin_sync_delete;
DROP TRIGGER IF EXISTS reminder_sync_insert;
DROP TRIGGER IF EXISTS reminder_sync_update;
DROP TRIGGER IF EXISTS reminder_sync_delete;
DROP TRIGGER IF EXISTS snippets_sync_insert;
DROP TRIGGER IF EXISTS snippets_sync_update;
DROP TRIGGER IF EXISTS snippets_sync_delete;
DROP TRIGGER IF EXISTS todo_sync_insert;
DROP TRIGGER IF EXISTS todo_sync_update;
DROP TRIGGER IF EXISTS todo_sync_delete;
DROP TRIGGER IF EXISTS workspaces_sync_insert;
DROP TRIGGER IF EXISTS workspaces_sync_update;
DROP TRIGGER IF EXISTS workspaces_sync_delete;
DROP TRIGGER IF EXISTS user_preference_sync_insert;
DROP TRIGGER IF EXISTS user_preference_sync_update;
DROP TRIGGER IF EXISTS user_preference_sync_delete;
DROP TABLE IF EXISTS sync_queue;
"#,
                )
                .await?;
        } else if db_backend == DbBackend::Postgres {
            db_connection
                .execute_unprepared(
                    r#"
DROP TRIGGER IF EXISTS bookmarks_sync_trigger ON bookmark;
DROP TRIGGER IF EXISTS notes_sync_trigger ON notes;
DROP TRIGGER IF EXISTS recycle_bin_sync_trigger ON recycle_bin;
DROP TRIGGER IF EXISTS reminder_sync_trigger ON reminder;
DROP TRIGGER IF EXISTS snippets_sync_trigger ON snippets;
DROP TRIGGER IF EXISTS todo_sync_trigger ON todo;
DROP TRIGGER IF EXISTS workspaces_sync_trigger ON workspaces;
DROP TRIGGER IF EXISTS user_preference_sync_trigger ON user_preference;
DROP FUNCTION IF EXISTS enqueue_sync;
DROP TABLE IF EXISTS sync_queue;
"#,
                )
                .await?;
        } else if db_backend == DbBackend::MySql {
            db_connection
                .execute_unprepared(
                    r#"
DROP TRIGGER IF EXISTS bookmark_sync_insert;
DROP TRIGGER IF EXISTS bookmark_sync_update;
DROP TRIGGER IF EXISTS bookmark_sync_delete;
DROP TRIGGER IF EXISTS notes_sync_insert;
DROP TRIGGER IF EXISTS notes_sync_update;
DROP TRIGGER IF EXISTS notes_sync_delete;
DROP TRIGGER IF EXISTS recycle_bin_sync_insert;
DROP TRIGGER IF EXISTS recycle_bin_sync_update;
DROP TRIGGER IF EXISTS recycle_bin_sync_delete;
DROP TRIGGER IF EXISTS reminder_sync_insert;
DROP TRIGGER IF EXISTS reminder_sync_update;
DROP TRIGGER IF EXISTS reminder_sync_delete;
DROP TRIGGER IF EXISTS snippets_sync_insert;
DROP TRIGGER IF EXISTS snippets_sync_update;
DROP TRIGGER IF EXISTS snippets_sync_delete;
DROP TRIGGER IF EXISTS todo_sync_insert;
DROP TRIGGER IF EXISTS todo_sync_update;
DROP TRIGGER IF EXISTS todo_sync_delete;
DROP TRIGGER IF EXISTS workspaces_sync_insert;
DROP TRIGGER IF EXISTS workspaces_sync_update;
DROP TRIGGER IF EXISTS workspaces_sync_delete;
DROP TRIGGER IF EXISTS user_preference_sync_insert;
DROP TRIGGER IF EXISTS user_preference_sync_update;
DROP TRIGGER IF EXISTS user_preference_sync_delete;
DROP TABLE IF EXISTS sync_queue;
"#,
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
