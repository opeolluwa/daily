-- ============================================
-- m20261116_000000_switch_identifiers_to_ulid
-- ============================================


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

CREATE OR REPLACE FUNCTION daily_ulid_fresh()
RETURNS text LANGUAGE sql VOLATILE AS $$
    SELECT daily_uuid_to_ulid(gen_random_uuid());
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

CREATE TEMP TABLE _daily_ulid_columns (
    table_name text NOT NULL,
    column_name text NOT NULL,
    mode text NOT NULL
) ON COMMIT DROP;

INSERT INTO _daily_ulid_columns(table_name, column_name, mode) VALUES
    ('notes', 'identifier', 'ulid'),
    ('notes', 'workspace_identifier', 'ulid'),
    ('task', 'identifier', 'ulid'),
    ('task', 'workspace_identifier', 'ulid'),
    ('bookmark', 'identifier', 'ulid'),
    ('bookmark', 'workspace_identifier', 'ulid'),
    ('snippets', 'identifier', 'ulid'),
    ('snippets', 'workspace_identifier', 'ulid'),
    ('reminder', 'identifier', 'ulid'),
    ('reminder', 'workspace_identifier', 'ulid'),
    ('paste_bins', 'identifier', 'ulid'),
    ('paste_bins', 'workspace_identifier', 'ulid'),
    ('whiteboards', 'identifier', 'ulid'),
    ('whiteboards', 'workspace_identifier', 'ulid'),
    ('workspace_profiles', 'identifier', 'ulid'),
    ('workspace_profiles', 'workspace_identifier', 'ulid'),
    ('workspace_members', 'identifier', 'ulid'),
    ('workspace_members', 'workspace_identifier', 'ulid'),
    ('workspace_members', 'user_identifier', 'ulid'),
    ('notifications', 'identifier', 'ulid'),
    ('notifications', 'workspace_identifier', 'ulid'),
    ('recycle_bin', 'identifier', 'ulid'),
    ('recycle_bin', 'item_id', 'ulid'),
    ('recycle_bin', 'workspace_identifier', 'ulid'),
    ('note_categories', 'identifier', 'ulid'),
    ('user_preferences', 'identifier', 'ulid'),
    ('users', 'identifier', 'ulid'),
    ('one_time_password', 'identifier', 'ulid'),
    ('one_time_password', 'user_identifier', 'ulid'),
    ('revoked_token', 'identifier', 'ulid'),
    ('revoked_token', 'jti', 'ulid'),
    ('revoked_token', 'user_identifier', 'ulid'),
    ('invitation', 'identifier', 'ulid'),
    ('invitation', 'workspace_identifier', 'ulid'),
    ('workspaces', 'identifier', 'ulid'),
    ('workspaces', 'user_identifier', 'ulid'),
    ('whiteboard_files', 'identifier', 'opaque'),
    ('whiteboard_files', 'whiteboard_identifier', 'ulid'),
    ('sync_queue', 'identifier', 'fresh'),
    ('sync_queue', 'record_identifier', 'ulid'),
    ('sync_queue', 'workspace_identifier', 'ulid'),
    ('sync_state', 'workspace_identifier', 'ulid');

CREATE TEMP TABLE _daily_ulid_parents (table_name text PRIMARY KEY) ON COMMIT DROP;
INSERT INTO _daily_ulid_parents(table_name) VALUES
    ('notes'), ('task'), ('bookmark'), ('snippets'), ('reminder'), ('paste_bins'),
    ('whiteboards'), ('workspace_profiles'), ('workspace_members'), ('notifications'),
    ('recycle_bin'), ('note_categories'), ('user_preferences'), ('users'),
    ('one_time_password'), ('revoked_token'), ('invitation'), ('workspaces');

CREATE TEMP TABLE _daily_ulid_relations (
    parent_table text NOT NULL,
    parent_column text NOT NULL,
    child_table text NOT NULL,
    child_column text NOT NULL
) ON COMMIT DROP;
INSERT INTO _daily_ulid_relations VALUES
    ('workspaces', 'identifier', 'notes', 'workspace_identifier'),
    ('workspaces', 'identifier', 'task', 'workspace_identifier'),
    ('workspaces', 'identifier', 'bookmark', 'workspace_identifier'),
    ('workspaces', 'identifier', 'snippets', 'workspace_identifier'),
    ('workspaces', 'identifier', 'reminder', 'workspace_identifier'),
    ('workspaces', 'identifier', 'paste_bins', 'workspace_identifier'),
    ('workspaces', 'identifier', 'whiteboards', 'workspace_identifier'),
    ('workspaces', 'identifier', 'workspace_profiles', 'workspace_identifier'),
    ('workspaces', 'identifier', 'workspace_members', 'workspace_identifier'),
    ('workspaces', 'identifier', 'notifications', 'workspace_identifier'),
    ('workspaces', 'identifier', 'recycle_bin', 'workspace_identifier'),
    ('workspaces', 'identifier', 'invitation', 'workspace_identifier'),
    ('users', 'identifier', 'workspaces', 'user_identifier'),
    ('users', 'identifier', 'workspace_members', 'user_identifier'),
    ('users', 'identifier', 'one_time_password', 'user_identifier'),
    ('users', 'identifier', 'revoked_token', 'user_identifier'),
    ('whiteboards', 'identifier', 'whiteboard_files', 'whiteboard_identifier');

CREATE TEMP TABLE _daily_ulid_fks ON COMMIT DROP AS
SELECT conrelid::regclass::text AS table_name,
       conname,
       pg_get_constraintdef(oid) AS definition
FROM pg_constraint
WHERE contype = 'f' AND connamespace = 'public'::regnamespace;

DO $$
DECLARE
    fk record;
BEGIN
    FOR fk IN SELECT * FROM _daily_ulid_fks LOOP
        EXECUTE format('ALTER TABLE %s DROP CONSTRAINT %I', fk.table_name, fk.conname);
    END LOOP;
END;
$$;

DO $$
DECLARE
    item record;
BEGIN
    FOR item IN SELECT * FROM _daily_ulid_columns ORDER BY table_name, column_name LOOP
        EXECUTE format(
            'ALTER TABLE %I ALTER COLUMN %I TYPE text USING %I::text',
            item.table_name, item.column_name, item.column_name
        );
    END LOOP;
END;
$$;

DO $$
DECLARE
    parent record;
    dup record;
    relation record;
    winner tid;
    loser tid;
    i integer;
BEGIN
    FOR parent IN SELECT table_name FROM _daily_ulid_parents LOOP
        FOR dup IN EXECUTE format(
            'SELECT identifier, array_agg(ctid ORDER BY ctid) AS rows
             FROM %I GROUP BY identifier HAVING count(*) > 1',
            parent.table_name
        ) LOOP
            winner := dup.rows[1];
            FOR i IN 2..array_length(dup.rows, 1) LOOP
                loser := dup.rows[i];
                FOR relation IN
                    SELECT * FROM _daily_ulid_relations WHERE parent_table = parent.table_name
                LOOP
                    EXECUTE format(
                        'UPDATE %I SET %I = (SELECT %I FROM %I WHERE ctid = $1)
                         WHERE %I = (SELECT %I FROM %I WHERE ctid = $2)',
                        relation.child_table, relation.child_column,
                        relation.parent_column, parent.table_name,
                        relation.child_column, relation.parent_column, parent.table_name
                    ) USING winner, loser;
                END LOOP;
                EXECUTE format('DELETE FROM %I WHERE ctid = $1', parent.table_name)
                USING loser;
            END LOOP;
        END LOOP;
    END LOOP;

    DELETE FROM sync_state a USING sync_state b
    WHERE a.workspace_identifier = b.workspace_identifier
      AND (a.last_cursor < b.last_cursor
           OR (a.last_cursor = b.last_cursor AND a.ctid > b.ctid));
END;
$$;

DO $$
DECLARE
    item record;
BEGIN
    FOR item IN SELECT * FROM _daily_ulid_columns ORDER BY table_name, column_name LOOP
        IF item.mode = 'fresh' THEN
            EXECUTE format(
                'ALTER TABLE %I ALTER COLUMN %I TYPE varchar(26) USING daily_ulid_fresh()',
                item.table_name, item.column_name
            );
        ELSIF item.mode = 'opaque' THEN
            EXECUTE format(
                'ALTER TABLE %I ALTER COLUMN %I TYPE varchar(64) USING %I',
                item.table_name, item.column_name, item.column_name
            );
        ELSE
            EXECUTE format(
                'ALTER TABLE %I ALTER COLUMN %I TYPE varchar(26) USING daily_text_to_ulid(%I)',
                item.table_name, item.column_name, item.column_name
            );
        END IF;
    END LOOP;
END;
$$;

UPDATE recycle_bin
SET payload = daily_ulidify_json(payload::jsonb)::text
WHERE payload IS NOT NULL AND payload ~ '^\s*[\{\[]';

DO $$
DECLARE
    fk record;
BEGIN
    FOR fk IN SELECT * FROM _daily_ulid_fks LOOP
        EXECUTE format('ALTER TABLE %s ADD CONSTRAINT %I %s', fk.table_name, fk.conname, fk.definition);
    END LOOP;
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

CREATE OR REPLACE FUNCTION enqueue_sync()
RETURNS TRIGGER AS $$
DECLARE
    applying text;
    rec_id text;
    ws_id text;
BEGIN
    SELECT value INTO applying FROM sync_meta WHERE key = 'applying';
    IF applying IS NOT NULL AND applying <> '' THEN
        RETURN COALESCE(NEW, OLD);
    END IF;

    IF TG_OP = 'DELETE' THEN
        rec_id := OLD.identifier::text;
        ws_id := COALESCE(to_jsonb(OLD) ->> 'workspace_identifier', OLD.identifier::text);
    ELSE
        rec_id := NEW.identifier::text;
        ws_id := COALESCE(to_jsonb(NEW) ->> 'workspace_identifier', NEW.identifier::text);
    END IF;

    INSERT INTO sync_queue(identifier, table_name, record_identifier, operation, workspace_identifier, created_at, attempts)
    VALUES (daily_ulid_fresh(), TG_TABLE_NAME, rec_id, TG_OP, ws_id, NOW(), 0);
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER notes_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON notes FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER task_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON task FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER bookmark_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON bookmark FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER snippets_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON snippets FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER reminder_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON reminder FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER paste_bins_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON paste_bins FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER whiteboards_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON whiteboards FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER workspace_profiles_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON workspace_profiles FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
CREATE TRIGGER workspaces_sync_trigger AFTER INSERT OR UPDATE OR DELETE ON workspaces FOR EACH ROW EXECUTE FUNCTION enqueue_sync();
;

