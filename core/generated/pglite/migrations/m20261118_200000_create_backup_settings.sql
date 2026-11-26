-- ============================================
-- m20261118_200000_create_backup_settings
-- ============================================

SELECT 1 FROM pg_type WHERE typname = 'backup_mode';

CREATE TYPE "backup_mode" AS ENUM ('local_only', 'cloud_only', 'local_first', 'cloud_first');

CREATE TABLE IF NOT EXISTS "backup_settings" ( "identifier" uuid NOT NULL PRIMARY KEY, "user_identifier" char(36), "server_url" varchar NOT NULL DEFAULT '', "last_sync" timestamp with time zone, "backup_mode" backup_mode NOT NULL DEFAULT 'local_only', "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );
