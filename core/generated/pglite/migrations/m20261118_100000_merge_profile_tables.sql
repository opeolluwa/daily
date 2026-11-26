-- ============================================
-- m20261118_100000_merge_profile_tables
-- ============================================

CREATE TABLE IF NOT EXISTS "user_profile" ( "identifier" uuid NOT NULL PRIMARY KEY, "first_name" varchar NOT NULL DEFAULT '', "last_name" varchar NOT NULL DEFAULT '', "email" varchar NOT NULL DEFAULT '', "profile_picture" varchar, "created_at" timestamp with time zone NOT NULL, "updated_at" timestamp with time zone NOT NULL );

INSERT INTO user_profile (identifier, first_name, last_name, email, profile_picture, created_at, updated_at)
             SELECT
                 wp.identifier,
                 COALESCE(wp.first_name, ''),
                 COALESCE(wp.last_name, ''),
                 COALESCE(up.master_email, ''),
                 wp.profile_picture,
                 wp.created_at,
                 wp.updated_at
             FROM workspace_profiles wp
             LEFT JOIN user_preferences up ON 1=1
             WHERE wp.deleted_at IS NULL
             ORDER BY wp.updated_at DESC
             LIMIT 1;

DROP TABLE IF EXISTS "workspace_profiles";

DROP TABLE IF EXISTS "user_preferences";
