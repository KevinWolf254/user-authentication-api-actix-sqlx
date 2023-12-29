-- Table: SMS_GATEWAY_USER.PERMISSION

-- DROP TABLE IF EXISTS "SMS_GATEWAY_USER"."PERMISSION";

CREATE TABLE IF NOT EXISTS "SMS_GATEWAY_USER"."PERMISSION"
(
    permission_id smallint NOT NULL DEFAULT nextval('"SMS_GATEWAY_USER"."PERMISSION_permission_id_seq"'::regclass),
    name character varying COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp with time zone NOT NULL,
    CONSTRAINT "PERMISSION_pkey" PRIMARY KEY (permission_id),
    CONSTRAINT "PERMISSION_name_key" UNIQUE (name)
)