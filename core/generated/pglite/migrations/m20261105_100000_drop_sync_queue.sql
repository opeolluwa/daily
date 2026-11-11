-- ============================================
-- m20261105_100000_drop_sync_queue
-- ============================================


DROP TRIGGER IF EXISTS bookmarks_sync_trigger ON bookmark;
DROP TRIGGER IF EXISTS notes_sync_trigger ON notes;
DROP TRIGGER IF EXISTS recycle_bin_sync_trigger ON recycle_bin;
DROP TRIGGER IF EXISTS reminder_sync_trigger ON reminder;
DROP TRIGGER IF EXISTS snippets_sync_trigger ON snippets;
DROP TRIGGER IF EXISTS todo_sync_trigger ON todo;
DROP TRIGGER IF EXISTS workspaces_sync_trigger ON workspaces;
DROP TRIGGER IF EXISTS user_preference_sync_trigger ON workspace_profiles;
DROP FUNCTION IF EXISTS enqueue_sync;
DROP TABLE IF EXISTS sync_queue;
;

