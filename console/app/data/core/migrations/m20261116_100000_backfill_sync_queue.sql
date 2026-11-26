-- ============================================
-- m20261116_100000_backfill_sync_queue
-- ============================================

INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT daily_ulid_fresh(), 'workspaces', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       t.identifier, NOW(), 0
FROM "workspaces" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = 'workspaces' AND sq.record_identifier = t.identifier
);

INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT daily_ulid_fresh(), 'workspace_profiles', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       t.workspace_identifier, NOW(), 0
FROM "workspace_profiles" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = 'workspace_profiles' AND sq.record_identifier = t.identifier
) AND t.workspace_identifier IS NOT NULL;

INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT daily_ulid_fresh(), 'notes', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       t.workspace_identifier, NOW(), 0
FROM "notes" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = 'notes' AND sq.record_identifier = t.identifier
) AND t.workspace_identifier IS NOT NULL;

INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT daily_ulid_fresh(), 'task', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       t.workspace_identifier, NOW(), 0
FROM "task" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = 'task' AND sq.record_identifier = t.identifier
) AND t.workspace_identifier IS NOT NULL;

INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT daily_ulid_fresh(), 'bookmark', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       t.workspace_identifier, NOW(), 0
FROM "bookmark" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = 'bookmark' AND sq.record_identifier = t.identifier
) AND t.workspace_identifier IS NOT NULL;

INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT daily_ulid_fresh(), 'snippets', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       t.workspace_identifier, NOW(), 0
FROM "snippets" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = 'snippets' AND sq.record_identifier = t.identifier
) AND t.workspace_identifier IS NOT NULL;

INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT daily_ulid_fresh(), 'reminder', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       t.workspace_identifier, NOW(), 0
FROM "reminder" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = 'reminder' AND sq.record_identifier = t.identifier
) AND t.workspace_identifier IS NOT NULL;

INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT daily_ulid_fresh(), 'paste_bins', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       t.workspace_identifier, NOW(), 0
FROM "paste_bins" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = 'paste_bins' AND sq.record_identifier = t.identifier
) AND t.workspace_identifier IS NOT NULL;

INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
SELECT daily_ulid_fresh(), 'whiteboards', t.identifier,
       CASE WHEN t.deleted_at IS NOT NULL THEN 'DELETE' ELSE 'INSERT' END,
       t.workspace_identifier, NOW(), 0
FROM "whiteboards" t
WHERE NOT EXISTS (
    SELECT 1 FROM sync_queue sq
    WHERE sq.table_name = 'whiteboards' AND sq.record_identifier = t.identifier
) AND t.workspace_identifier IS NOT NULL;
