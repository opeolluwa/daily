-- ============================================
-- m20261111_000000_create_whiteboards
-- ============================================

CREATE TABLE IF NOT EXISTS "whiteboards" ( "identifier" uuid NOT NULL PRIMARY KEY, "title" varchar NULL, "payload" json NULL, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL, "workspace_identifier" uuid NULL, CONSTRAINT "fk_whiteboards_workspace_identifier" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE );

CREATE INDEX "idx_whiteboards_workspace_identifier" ON "whiteboards" ("workspace_identifier");

CREATE TABLE IF NOT EXISTS "whiteboard_files" ( "identifier" uuid NOT NULL PRIMARY KEY, "whiteboard_identifier" uuid NOT NULL, "mime_type" varchar NOT NULL, "data" text NOT NULL, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL, CONSTRAINT "fk_whiteboard_files_whiteboard_identifier" FOREIGN KEY ("whiteboard_identifier") REFERENCES "whiteboards" ("identifier") ON DELETE CASCADE );

CREATE INDEX "idx_whiteboard_files_whiteboard_identifier" ON "whiteboard_files" ("whiteboard_identifier");

