-- ============================================
-- m20261105_000000_create_paste_bins
-- ============================================

ALTER TYPE "item_type" ADD VALUE IF NOT EXISTS 'paste_bin';;

CREATE TABLE IF NOT EXISTS "paste_bins" ( "identifier" uuid NOT NULL PRIMARY KEY, "title" varchar NULL, "content" text NOT NULL, "is_pinned" bool NOT NULL DEFAULT FALSE, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL, "workspace_identifier" uuid NULL, CONSTRAINT "fk_paste_bins_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );

CREATE INDEX "idx_paste_bins_workspace_identifier" ON "paste_bins" ("workspace_identifier");

