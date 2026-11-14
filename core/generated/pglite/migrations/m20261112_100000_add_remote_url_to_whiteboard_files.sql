-- ============================================
-- m20261112_100000_add_remote_url_to_whiteboard_files
-- ============================================

ALTER TABLE "whiteboard_files" ADD COLUMN "remote_url" text NULL;

