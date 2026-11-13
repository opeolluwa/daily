-- ============================================
-- m20261113_100000_add_end_date_to_reminder
-- ============================================

ALTER TABLE "reminder" ADD COLUMN "end_date" date_text NULL;

