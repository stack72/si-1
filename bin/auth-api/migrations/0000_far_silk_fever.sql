CREATE TABLE IF NOT EXISTS "tos_agreements" (
	"id" char(26) PRIMARY KEY NOT NULL,
	"user_id" char(26) NOT NULL,
	"tos_version_id" text NOT NULL,
	"timestamp" timestamp DEFAULT now() NOT NULL,
	"ip_address" text NOT NULL
);

CREATE TABLE IF NOT EXISTS "users" (
	"id" char(26) PRIMARY KEY NOT NULL,
	"auth_0_id" text NOT NULL,
	"auth0_details" jsonb,
	"nickname" text NOT NULL,
	"email" text NOT NULL,
	"email_verified" boolean DEFAULT false,
	"first_name" text,
	"last_name" text,
	"picture_url" text,
	"discord_username" text,
	"github_username" text,
	"onboarding_details" jsonb
);

CREATE TABLE IF NOT EXISTS "workspaces" (
	"id" char(26) PRIMARY KEY NOT NULL,
	"instance_env_type" text NOT NULL,
	"instance_url" text,
	"display_name" text,
	"creator_user_id" char(26)
);

DO $$ BEGIN
 ALTER TABLE "tos_agreements" ADD CONSTRAINT "tos_agreements_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
 ALTER TABLE "workspaces" ADD CONSTRAINT "workspaces_creator_user_id_users_id_fk" FOREIGN KEY ("creator_user_id") REFERENCES "users"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;

CREATE INDEX IF NOT EXISTS "tos_agreements_user_id_index" ON "tos_agreements" ("user_id");
CREATE INDEX IF NOT EXISTS "users_email_index" ON "users" ("email");
CREATE UNIQUE INDEX IF NOT EXISTS "users_auth_0_id_index" ON "users" ("auth_0_id");