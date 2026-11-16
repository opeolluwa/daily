-- ============================================
-- m20261112_000000_rename_todo_table_to_task
-- ============================================

ALTER TABLE "todo" RENAME TO "task";

ALTER TYPE "todo_status" RENAME TO "task_status";

ALTER TYPE "item_type" RENAME VALUE 'todo' TO 'task';

