-- ============================================
-- m20261113_000000_add_recurrence_type_to_reminder
-- ============================================

SELECT 1 FROM pg_type WHERE typname = 'reminder_recurrence';

CREATE TYPE "reminder_recurrence" AS ENUM ('daily', 'weekly', 'biweekly', 'monthly', 'yearly', 'weekdays');

ALTER TABLE "reminder" ADD COLUMN IF NOT EXISTS "recurrence_type" reminder_recurrence NULL;

