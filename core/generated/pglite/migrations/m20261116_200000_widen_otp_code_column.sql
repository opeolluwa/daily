-- ============================================
-- m20261116_200000_widen_otp_code_column
-- ============================================

ALTER TABLE "one_time_password" ALTER COLUMN "code" TYPE varchar(72);
