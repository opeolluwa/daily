-- ============================================
-- m20261105_100000_drop_sync_queue
-- ============================================


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
;

