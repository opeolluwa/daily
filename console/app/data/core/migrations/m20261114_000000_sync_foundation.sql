-- ============================================
-- m20261114_000000_sync_foundation
-- ============================================

ALTER TABLE "notes" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;;

ALTER TABLE "task" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;;

ALTER TABLE "bookmark" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;;

ALTER TABLE "snippets" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;;

ALTER TABLE "reminder" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;;

ALTER TABLE "paste_bins" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;;

ALTER TABLE "whiteboards" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;;

ALTER TABLE "workspace_profiles" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;;

CREATE TABLE IF NOT EXISTS "sync_queue" ( "identifier" uuid NOT NULL PRIMARY KEY, "table_name" varchar NOT NULL, "record_identifier" varchar NOT NULL, "operation" varchar NOT NULL, "workspace_identifier" uuid NULL, "created_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP, "attempts" integer NOT NULL DEFAULT 0, "last_error" text NULL, "device_id" varchar NULL );

CREATE INDEX "idx_sync_queue_workspace" ON "sync_queue" ("workspace_identifier");

CREATE INDEX "idx_sync_queue_record" ON "sync_queue" ("table_name", "record_identifier");

CREATE TABLE IF NOT EXISTS "sync_state" ( "workspace_identifier" uuid NOT NULL PRIMARY KEY, "last_cursor" bigint NOT NULL DEFAULT 0, "last_synced_at" timestamp with time zone NULL, "last_error" text NULL );

CREATE TABLE IF NOT EXISTS "sync_meta" ( "key" varchar NOT NULL PRIMARY KEY, "value" text NOT NULL );

DROP TRIGGER IF EXISTS notes_sync_trigger ON "notes";;

DROP TRIGGER IF EXISTS task_sync_trigger ON "task";;

DROP TRIGGER IF EXISTS bookmark_sync_trigger ON "bookmark";;

DROP TRIGGER IF EXISTS snippets_sync_trigger ON "snippets";;

DROP TRIGGER IF EXISTS reminder_sync_trigger ON "reminder";;

DROP TRIGGER IF EXISTS paste_bins_sync_trigger ON "paste_bins";;

DROP TRIGGER IF EXISTS whiteboards_sync_trigger ON "whiteboards";;

DROP TRIGGER IF EXISTS workspace_profiles_sync_trigger ON "workspace_profiles";;

DROP TRIGGER IF EXISTS workspaces_sync_trigger ON "workspaces";;


CREATE OR REPLACE FUNCTION enqueue_sync()
RETURNS TRIGGER AS $$
DECLARE
    applying text;
    rec_id uuid;
    ws_id uuid;
BEGIN
    SELECT value INTO applying FROM sync_meta WHERE key = 'applying';
    IF applying IS NOT NULL AND applying <> '' THEN
        RETURN COALESCE(NEW, OLD);
    END IF;

    IF TG_OP = 'DELETE' THEN
        rec_id := OLD.identifier;
        ws_id := COALESCE((to_jsonb(OLD) ->> 'workspace_identifier')::uuid, OLD.identifier);
    ELSE
        rec_id := NEW.identifier;
        ws_id := COALESCE((to_jsonb(NEW) ->> 'workspace_identifier')::uuid, NEW.identifier);
    END IF;

    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    VALUES (gen_random_uuid(), TG_TABLE_NAME, rec_id::text, TG_OP, ws_id, NOW(), 0);

    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;
;

CREATE TRIGGER notes_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON "notes"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();;

CREATE TRIGGER task_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON "task"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();;

CREATE TRIGGER bookmark_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON "bookmark"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();;

CREATE TRIGGER snippets_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON "snippets"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();;

CREATE TRIGGER reminder_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON "reminder"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();;

CREATE TRIGGER paste_bins_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON "paste_bins"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();;

CREATE TRIGGER whiteboards_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON "whiteboards"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();;

CREATE TRIGGER workspace_profiles_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON "workspace_profiles"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();;

CREATE TRIGGER workspaces_sync_trigger
                 AFTER INSERT OR UPDATE OR DELETE ON "workspaces"
                 FOR EACH ROW EXECUTE FUNCTION enqueue_sync();;

