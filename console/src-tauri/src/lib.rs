mod adapters;
mod commands;
mod errors;
mod secrets;
mod state;
use std::sync::Arc;

use kernel::{adapters::notifications::CreateNotification, DataEngine};
use tauri::Listener;
use tauri::Manager;

use crate::state::app::AppState;

const EVENT_NOTIFICATION_RECEIVED: &str = "notification:received";

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                let _ = app
                    .get_webview_window("main")
                    .expect("no main window")
                    .set_focus();
            }))
            .plugin(tauri_plugin_window_state::Builder::new().build());
    }

    builder
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            app.listen(EVENT_NOTIFICATION_RECEIVED, |event| {
                if let Ok(payload) = serde_json::from_str::<CreateNotification>(event.payload()) {
                    println!("downloading {:#?}", payload);
                }
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_handle = app.handle().clone();

            // Initialize state synchronously so it is managed before the
            // frontend window can invoke any Tauri commands.
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    let app_data_dir = app_handle
                        .path()
                        .app_data_dir()
                        .expect("failed to resolve app data dir");

                    std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");

                    let db_path = match std::env::var("ALMONDS_DB_PATH") {
                        Ok(path) => std::path::PathBuf::from(path),
                        Err(_) => app_data_dir.join("almonds.db"),
                    };

                    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
                    log::debug!("Database path resolved");
                    let kernel = DataEngine::new(&db_url)
                        .await
                        .expect("failed to initialize kernel");

                    kernel
                        .run_migrations()
                        .await
                        .expect("failed to run migrations");

                    let conn = Arc::new(kernel.connection().clone());

                    let state = AppState::new(conn).await;

                    app_handle.manage(state);

                    let state = app_handle.state::<AppState>();
                    commands::cloud_storage::load_saved_storage(&app_handle, state.inner()).await;
                })
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::bookmarks::create_bookmark,
            commands::bookmarks::delete_bookmark,
            commands::bookmarks::duplicate_bookmark,
            commands::bookmarks::get_all_bookmarks,
            commands::bookmarks::get_bookmark,
            commands::bookmarks::get_bookmarks_by_tag,
            commands::bookmarks::get_recently_added_bookmarks,
            commands::bookmarks::transfer_bookmark,
            commands::bookmarks::update_bookmark,
            commands::notifications::create_notification,
            commands::notifications::delete_notification,
            commands::notifications::get_all_notifications,
            commands::notifications::get_notification,
            commands::notifications::get_notifications_by_type,
            commands::notifications::mark_notification_as_read,
            commands::notes::create_note,
            commands::notes::delete_note,
            commands::notes::duplicate_note,
            commands::notes::get_all_notes,
            commands::notes::get_note,
            commands::notes::get_recently_added_notes,
            commands::notes::transfer_note,
            commands::notes::update_note,
            commands::notes::export_notes_as_pdf,
            commands::paste_bins::create_paste_bin,
            commands::paste_bins::delete_paste_bin,
            commands::paste_bins::duplicate_paste_bin,
            commands::paste_bins::get_all_paste_bins,
            commands::paste_bins::get_paste_bin,
            commands::paste_bins::get_recently_added_paste_bins,
            commands::paste_bins::transfer_paste_bin,
            commands::paste_bins::update_paste_bin,
            commands::whiteboards::create_whiteboard,
            commands::whiteboards::delete_whiteboard,
            commands::whiteboards::delete_whiteboard_file,
            commands::whiteboards::get_all_whiteboards,
            commands::whiteboards::get_recently_added_whiteboards,
            commands::whiteboards::get_whiteboard,
            commands::whiteboards::get_whiteboard_files,
            commands::whiteboards::save_whiteboard_file,
            commands::whiteboards::transfer_whiteboard,
            commands::whiteboards::update_whiteboard,
            commands::reminder::create_reminder,
            commands::reminder::delete_reminder,
            commands::reminder::duplicate_reminder,
            commands::reminder::get_all_reminders,
            commands::reminder::get_reminder,
            commands::reminder::transfer_reminder,
            commands::reminder::update_reminder,
            commands::recycle_bin::create_recycle_bin_entry,
            commands::recycle_bin::get_all_recycle_bin_entries,
            commands::recycle_bin::get_recycle_bin_entry,
            commands::recycle_bin::get_recycle_bin_entries_by_type,
            commands::recycle_bin::purge_all_recycle_bin_entries,
            commands::recycle_bin::purge_recycle_bin_entry,
            commands::recycle_bin::restore_recycle_bin_entry,
            commands::snippets::create_snippet,
            commands::snippets::delete_snippet,
            commands::snippets::duplicate_snippet,
            commands::snippets::get_all_snippets,
            commands::snippets::get_recently_added_snippet,
            commands::snippets::get_snippet,
            commands::snippets::transfer_snippet,
            commands::snippets::update_snippet,
            commands::tasks::change_task_priority,
            commands::tasks::create_task,
            commands::tasks::delete_task,
            commands::tasks::duplicate_task,
            commands::tasks::get_all_tasks,
            commands::tasks::get_task,
            commands::tasks::mark_task_done,
            commands::tasks::transfer_task,
            commands::tasks::update_task,
            commands::tasks::update_task_due_date,
            commands::workspace_profiles::create_workspace_profile,
            commands::workspace_profiles::duplicate_workspace_profile,
            commands::workspace_profiles::get_workspace_profile,
            commands::workspace_profiles::transfer_workspace_profile,
            commands::workspace_profiles::update_workspace_profile,
            commands::workspaces::create_workspace,
            commands::workspaces::delete_workspace,
            commands::workspaces::get_workspace_by_id,
            commands::workspaces::list_workspaces,
            commands::workspaces::update_workspace,
            commands::workspaces::verify_workspace_password,
            commands::moodboard::save_moodboard_image,
            commands::moodboard::list_moodboard_images,
            commands::moodboard::delete_moodboard_image,
            commands::cloud_storage::save_cloud_storage,
            commands::cloud_storage::get_cloud_storage_status,
            commands::cloud_storage::disable_cloud_storage,
            commands::cloud_storage::clear_cloud_storage,
            commands::cloud_storage::test_cloud_storage,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
