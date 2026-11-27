-- ============================================
-- m20261127_134557_add_category_to_task
-- ============================================

ALTER TABLE "task" ADD COLUMN IF NOT EXISTS "category_identifier" varchar;

ALTER TABLE "task" ADD CONSTRAINT "fk_task_category_identifier"
  FOREIGN KEY ("category_identifier") REFERENCES "task_category" ("identifier") ON DELETE SET NULL;
