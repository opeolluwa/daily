-- ============================================
-- m20261108_113612_add_status_to_tasks
-- ============================================

SELECT 1 FROM pg_type WHERE typname = 'todo_status';

CREATE TYPE "todo_status" AS ENUM ('planned', 'in_progress', 'done');

ALTER TABLE "todo" ADD COLUMN IF NOT EXISTS "status" todo_status NOT NULL DEFAULT 'planned';

UPDATE todo SET status = 'done' WHERE done = TRUE;

