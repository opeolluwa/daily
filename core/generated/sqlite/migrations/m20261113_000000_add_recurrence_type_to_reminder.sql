-- ============================================
-- m20261113_000000_add_recurrence_type_to_reminder
-- ============================================

ALTER TABLE "reminder" ADD COLUMN "recurrence_type" enum_text NULL;

