ALTER TABLE "user" ADD COLUMN IF NOT EXISTS "webhook_url" TEXT;

ALTER TABLE "website" ADD COLUMN IF NOT EXISTS "component" TEXT;

ALTER TABLE "website_tick" ADD COLUMN IF NOT EXISTS "http_status" INTEGER;

CREATE TABLE "incident" (
    "id" TEXT NOT NULL,
    "website_id" TEXT NOT NULL,
    "region_id" TEXT NOT NULL,
    "started_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "ended_at" TIMESTAMP(3),

    CONSTRAINT "incident_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "maintenance" (
    "id" TEXT NOT NULL,
    "website_id" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "starts_at" TIMESTAMP(3) NOT NULL,
    "ends_at" TIMESTAMP(3),
    "status" TEXT NOT NULL DEFAULT 'scheduled',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "maintenance_pkey" PRIMARY KEY ("id")
);
