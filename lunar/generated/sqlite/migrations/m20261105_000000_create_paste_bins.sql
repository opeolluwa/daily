-- ============================================
-- m20261105_000000_create_paste_bins
-- ============================================

CREATE TABLE IF NOT EXISTS "paste_bins" ( "identifier" uuid_text NOT NULL PRIMARY KEY, "title" varchar NULL, "content" text NOT NULL, "is_pinned" boolean NOT NULL DEFAULT FALSE, "created_at" timestamp_with_timezone_text NOT NULL, "updated_at" timestamp_with_timezone_text NOT NULL, "workspace_identifier" uuid_text NULL, FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );

CREATE INDEX "idx_paste_bins_workspace_identifier" ON "paste_bins" ("workspace_identifier");

