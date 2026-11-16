-- ============================================
-- m20261115_000000_workspace_tombstones
-- ============================================

ALTER TABLE "workspaces" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;;

CREATE INDEX IF NOT EXISTS "idx_workspaces_deleted_at" ON "workspaces" ("deleted_at");

