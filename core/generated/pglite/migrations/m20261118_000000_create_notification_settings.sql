-- ============================================
-- m20261118_000000_create_notification_settings
-- ============================================

CREATE TABLE IF NOT EXISTS "notification_settings" ( "identifier" uuid NOT NULL PRIMARY KEY, "tasks_due" bool NOT NULL DEFAULT TRUE, "reminders" bool NOT NULL DEFAULT TRUE, "sync_complete" bool NOT NULL DEFAULT FALSE, "app_updates" bool NOT NULL DEFAULT TRUE, "user_identifier" char(36), "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );
