use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    fn use_transaction(&self) -> Option<bool> {
        Some(true)
    }

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() != DbBackend::Postgres {
            return Ok(());
        }
        manager
            .get_connection()
            .execute_unprepared(POSTGRES_UP)
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

const POSTGRES_UP: &str = r##"
CREATE OR REPLACE FUNCTION daily_uuid_to_ulid(input uuid)
RETURNS text LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE
    alphabet CONSTANT text := '0123456789ABCDEFGHJKMNPQRSTVWXYZ';
    padded bit(130);
    idx integer;
    result text := '';
    i integer;
    j integer;
BEGIN
    IF input IS NULL THEN
        RETURN NULL;
    END IF;
    padded := B'00' || ('x' || replace(input::text, '-', ''))::bit(128);
    FOR i IN 0..25 LOOP
        idx := 0;
        FOR j IN 0..4 LOOP
            idx := idx * 2 + get_bit(padded, i * 5 + j);
        END LOOP;
        result := result || substr(alphabet, idx + 1, 1);
    END LOOP;
    RETURN result;
END;
$$;

CREATE OR REPLACE FUNCTION daily_text_to_ulid(input text)
RETURNS text LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE
    value text;
    compact text;
BEGIN
    IF input IS NULL THEN
        RETURN NULL;
    END IF;
    value := btrim(input);
    IF value = '' THEN
        RETURN value;
    END IF;
    IF length(value) = 26 AND value ~* '^[0-9A-HJKMNP-TV-Z]{26}$' THEN
        RETURN upper(value);
    END IF;
    compact := replace(lower(value), '-', '');
    IF length(compact) = 32 AND compact ~ '^[0-9a-f]{32}$' THEN
        RETURN daily_uuid_to_ulid((
            substr(compact, 1, 8) || '-' || substr(compact, 9, 4) || '-' ||
            substr(compact, 13, 4) || '-' || substr(compact, 17, 4) || '-' ||
            substr(compact, 21, 12)
        )::uuid);
    END IF;
    RETURN value;
END;
$$;

CREATE OR REPLACE FUNCTION daily_ulidify_json(input jsonb)
RETURNS jsonb LANGUAGE plpgsql IMMUTABLE AS $$
DECLARE
    result jsonb;
BEGIN
    IF input IS NULL THEN
        RETURN NULL;
    END IF;
    IF jsonb_typeof(input) = 'object' THEN
        SELECT jsonb_object_agg(
            key,
            CASE
                WHEN key IN ('identifier', 'workspace_identifier', 'record_identifier',
                             'item_id', 'item_identifier', 'user_identifier', 'jti',
                             'previous_workspace_identifier', 'target_workspace_identifier')
                     AND jsonb_typeof(value) = 'string'
                    THEN to_jsonb(daily_text_to_ulid(value #>> '{}'))
                WHEN jsonb_typeof(value) IN ('object', 'array')
                    THEN daily_ulidify_json(value)
                ELSE value
            END
        ) INTO result
        FROM jsonb_each(input);
        RETURN COALESCE(result, '{}'::jsonb);
    ELSIF jsonb_typeof(input) = 'array' THEN
        SELECT jsonb_agg(
            CASE
                WHEN jsonb_typeof(value) IN ('object', 'array')
                    THEN daily_ulidify_json(value)
                ELSE value
            END
        ) INTO result
        FROM jsonb_array_elements(input);
        RETURN COALESCE(result, '[]'::jsonb);
    END IF;
    RETURN input;
END;
$$;

DROP TRIGGER IF EXISTS notes_sync_trigger ON notes;
DROP TRIGGER IF EXISTS task_sync_trigger ON task;
DROP TRIGGER IF EXISTS bookmark_sync_trigger ON bookmark;
DROP TRIGGER IF EXISTS snippets_sync_trigger ON snippets;
DROP TRIGGER IF EXISTS reminder_sync_trigger ON reminder;
DROP TRIGGER IF EXISTS paste_bins_sync_trigger ON paste_bins;
DROP TRIGGER IF EXISTS whiteboards_sync_trigger ON whiteboards;
DROP TRIGGER IF EXISTS workspace_profiles_sync_trigger ON workspace_profiles;
DROP TRIGGER IF EXISTS workspaces_sync_trigger ON workspaces;
DROP TRIGGER IF EXISTS notes_sync_log_trigger ON notes;
DROP TRIGGER IF EXISTS task_sync_log_trigger ON task;
DROP TRIGGER IF EXISTS bookmark_sync_log_trigger ON bookmark;
DROP TRIGGER IF EXISTS snippets_sync_log_trigger ON snippets;
DROP TRIGGER IF EXISTS reminder_sync_log_trigger ON reminder;
DROP TRIGGER IF EXISTS paste_bins_sync_log_trigger ON paste_bins;
DROP TRIGGER IF EXISTS whiteboards_sync_log_trigger ON whiteboards;
DROP TRIGGER IF EXISTS workspace_profiles_sync_log_trigger ON workspace_profiles;
DROP TRIGGER IF EXISTS workspaces_sync_log_trigger ON workspaces;
DROP FUNCTION IF EXISTS enqueue_sync();
DROP FUNCTION IF EXISTS log_sync_change();

ALTER TABLE sync_log
    ALTER COLUMN workspace_identifier TYPE varchar(26)
        USING daily_text_to_ulid(workspace_identifier::text),
    ALTER COLUMN record_identifier TYPE varchar(26)
        USING daily_text_to_ulid(record_identifier::text);

UPDATE sync_log
SET payload = daily_ulidify_json(payload)
WHERE payload IS NOT NULL;

CREATE OR REPLACE FUNCTION log_sync_change()
RETURNS TRIGGER AS $$
DECLARE
    rec_id text;
    ws_id text;
BEGIN
    IF TG_OP = 'DELETE' THEN
        rec_id := OLD.identifier::text;
        ws_id := COALESCE(to_jsonb(OLD) ->> 'workspace_identifier', OLD.identifier::text);
    ELSE
        rec_id := NEW.identifier::text;
        ws_id := COALESCE(to_jsonb(NEW) ->> 'workspace_identifier', NEW.identifier::text);
    END IF;

    INSERT INTO sync_log(workspace_identifier, table_name, record_identifier, operation, payload)
    VALUES (
        ws_id,
        TG_TABLE_NAME,
        rec_id,
        TG_OP,
        CASE WHEN TG_OP = 'DELETE' THEN to_jsonb(OLD) ELSE to_jsonb(NEW) END
    );
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER notes_sync_log_trigger AFTER INSERT OR UPDATE OR DELETE ON notes FOR EACH ROW EXECUTE FUNCTION log_sync_change();
CREATE TRIGGER task_sync_log_trigger AFTER INSERT OR UPDATE OR DELETE ON task FOR EACH ROW EXECUTE FUNCTION log_sync_change();
CREATE TRIGGER bookmark_sync_log_trigger AFTER INSERT OR UPDATE OR DELETE ON bookmark FOR EACH ROW EXECUTE FUNCTION log_sync_change();
CREATE TRIGGER snippets_sync_log_trigger AFTER INSERT OR UPDATE OR DELETE ON snippets FOR EACH ROW EXECUTE FUNCTION log_sync_change();
CREATE TRIGGER reminder_sync_log_trigger AFTER INSERT OR UPDATE OR DELETE ON reminder FOR EACH ROW EXECUTE FUNCTION log_sync_change();
CREATE TRIGGER paste_bins_sync_log_trigger AFTER INSERT OR UPDATE OR DELETE ON paste_bins FOR EACH ROW EXECUTE FUNCTION log_sync_change();
CREATE TRIGGER whiteboards_sync_log_trigger AFTER INSERT OR UPDATE OR DELETE ON whiteboards FOR EACH ROW EXECUTE FUNCTION log_sync_change();
CREATE TRIGGER workspace_profiles_sync_log_trigger AFTER INSERT OR UPDATE OR DELETE ON workspace_profiles FOR EACH ROW EXECUTE FUNCTION log_sync_change();
CREATE TRIGGER workspaces_sync_log_trigger AFTER INSERT OR UPDATE OR DELETE ON workspaces FOR EACH ROW EXECUTE FUNCTION log_sync_change();
"##;
