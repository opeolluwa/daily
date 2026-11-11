-- ============================================
-- m20261108_113612_add_status_to_tasks
-- ============================================

ALTER TABLE "todo" ADD COLUMN "status" enum_text NOT NULL DEFAULT 'planned';

UPDATE todo SET status = 'done' WHERE done = TRUE;

