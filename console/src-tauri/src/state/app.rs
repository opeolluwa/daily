use std::sync::Arc;

use kernel::{
    filestore::FileStoreEngine,
    repositories::{
        backup_settings::BackupSettingsRepository, bookmarks::BookmarkRepository,
        notes::NotesRepository, notification_settings::NotificationSettingsRepository,
        notifications::NotificationRepository, paste_bins::PasteBinRepository, prelude::*,
        recycle_bin::RecycleBinRepository, reminder::ReminderRepository,
        snippets::SnippetRepository, sync::SyncRepository,
        task_categories::{TaskCategoryRepository, TaskCategoryRepositoryExt},
        tasks::TaskRepository,
        user_profile::UserProfileRepository, whiteboards::WhiteboardRepository,
        workspace::WorkspaceRepository,
    },
    sea_orm::DatabaseConnection,
};
use tokio::sync::RwLock;

pub struct AppState {
    pub backup_settings_repository: BackupSettingsRepository,
    pub bookmark_repository: BookmarkRepository,
    pub notes_repository: NotesRepository,
    pub notification_repository: NotificationRepository,
    pub notification_settings_repository: NotificationSettingsRepository,
    pub paste_bin_repository: PasteBinRepository,
    pub recycle_bin_repository: RecycleBinRepository,
    pub reminder_repository: ReminderRepository,
    pub snippet_repository: SnippetRepository,
    pub sync_repository: SyncRepository,
    pub task_category_repository: TaskCategoryRepository,
    pub task_repository: TaskRepository,
    pub whiteboards_repository: WhiteboardRepository,
    pub user_profile_repository: UserProfileRepository,
    pub workspace_repository: WorkspaceRepository,
    pub db: Arc<DatabaseConnection>,
    pub cloud_storage: Arc<RwLock<Option<FileStoreEngine>>>,
}

impl AppState {
    pub async fn new(conn: Arc<DatabaseConnection>) -> Self {
        let backup_settings_repository = BackupSettingsRepository::new(conn.clone());
        let bookmark_repository = BookmarkRepository::new(conn.clone());
        let notes_repository = NotesRepository::new(conn.clone());
        let notification_repository = NotificationRepository::new(conn.clone());
        let notification_settings_repository = NotificationSettingsRepository::new(conn.clone());
        let paste_bin_repository = PasteBinRepository::new(conn.clone());
        let recycle_bin_repository = RecycleBinRepository::new(conn.clone());
        let reminder_repository = ReminderRepository::new(conn.clone());
        let snippet_repository = SnippetRepository::new(conn.clone());
        let sync_repository = SyncRepository::new(conn.clone());
        let task_category_repository = TaskCategoryRepository::new(conn.clone());
        let task_repository = TaskRepository::new(conn.clone());
        let whiteboards_repository = WhiteboardRepository::new(conn.clone());
        let workspace_repository = WorkspaceRepository::new(conn.clone());
        let user_profile_repository = UserProfileRepository::new(conn.clone());

        AppState {
            backup_settings_repository,
            bookmark_repository,
            notes_repository,
            notification_repository,
            notification_settings_repository,
            paste_bin_repository,
            recycle_bin_repository,
            reminder_repository,
            snippet_repository,
            sync_repository,
            task_category_repository,
            task_repository,
            whiteboards_repository,
            workspace_repository,
            user_profile_repository,
            db: conn,
            cloud_storage: Arc::new(RwLock::new(None)),
        }
    }
}
