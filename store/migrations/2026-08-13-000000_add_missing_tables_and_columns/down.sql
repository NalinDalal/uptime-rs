ALTER TABLE "user" DROP COLUMN IF EXISTS "webhook_url";

ALTER TABLE "website" DROP COLUMN IF EXISTS "component";

ALTER TABLE "website_tick" DROP COLUMN IF EXISTS "http_status";

DROP TABLE IF EXISTS "incident";

DROP TABLE IF EXISTS "maintenance";
