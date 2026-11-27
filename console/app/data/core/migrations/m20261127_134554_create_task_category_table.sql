-- ============================================
-- m20261127_134554_create_task_category_table
-- ============================================

CREATE TABLE IF NOT EXISTS "task_category" (
  "identifier" varchar NOT NULL PRIMARY KEY,
  "name" varchar NOT NULL,
  "color" varchar NOT NULL,
  "workspace_identifier" varchar,
  "created_at" timestamp with time zone NOT NULL,
  "updated_at" timestamp with time zone NOT NULL,
  CONSTRAINT "fk_task_category_workspace" FOREIGN KEY ("workspace_identifier") REFERENCES "workspaces" ("identifier") ON DELETE CASCADE
);
