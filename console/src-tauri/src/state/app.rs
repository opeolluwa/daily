use std::sync::Arc;

use kernel::{
    filestore::FileStoreEngine,
    repositories::{
        bookmarks::BookmarkRepository, notes::NotesRepository,
        notifications::NotificationRepository, paste_bins::PasteBinRepository, prelude::*,
        recycle_bin::RecycleBinRepository,
        reminder::ReminderRepository, snippets::SnippetRepository, tasks::TaskRepository,
        whiteboards::WhiteboardRepository, workspace::WorkspaceRepository,
        workspace_profiles::WorkspaceProfileRepository,
    },
    sea_orm::DatabaseConnection,
};
use tokio::sync::RwLock;

pub struct AppState {
    pub bookmark_repository: BookmarkRepository,
    pub notes_repository: NotesRepository,
    pub notification_repository: NotificationRepository,
    pub paste_bin_repository: PasteBinRepository,
    pub recycle_bin_repository: RecycleBinRepository,
    pub reminder_repository: ReminderRepository,
    pub snippet_repository: SnippetRepository,
    pub task_repository: TaskRepository,
    pub whiteboards_repository: WhiteboardRepository,
    pub workspace_profile_repository: WorkspaceProfileRepository,
    pub workspace_repository: WorkspaceRepository,
    pub db: Arc<DatabaseConnection>,
    pub cloud_storage: Arc<RwLock<Option<FileStoreEngine>>>,
}

impl AppState {
    pub async fn new(conn: Arc<DatabaseConnection>) -> Self {
        let bookmark_repository = BookmarkRepository::new(conn.clone());
        let notes_repository = NotesRepository::new(conn.clone());
        let notification_repository = NotificationRepository::new(conn.clone());
        let paste_bin_repository = PasteBinRepository::new(conn.clone());
        let recycle_bin_repository = RecycleBinRepository::new(conn.clone());
        let reminder_repository = ReminderRepository::new(conn.clone());
        let snippet_repository = SnippetRepository::new(conn.clone());
        let task_repository = TaskRepository::new(conn.clone());
        let whiteboards_repository = WhiteboardRepository::new(conn.clone());
        let workspace_repository = WorkspaceRepository::new(conn.clone());
        let workspace_profile_repository = WorkspaceProfileRepository::new(conn.clone());

        AppState {
            bookmark_repository,
            notes_repository,
            notification_repository,
            paste_bin_repository,
            recycle_bin_repository,
            reminder_repository,
            snippet_repository,
            task_repository,
            whiteboards_repository,
            workspace_repository,
            workspace_profile_repository,
            db: conn,
            cloud_storage: Arc::new(RwLock::new(None)),
        }
    }
}
