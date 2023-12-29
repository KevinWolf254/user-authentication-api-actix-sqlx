-- Table: SMS_GATEWAY_USER.ROLE

-- DROP TABLE IF EXISTS "SMS_GATEWAY_USER"."ROLE";

CREATE TABLE IF NOT EXISTS "SMS_GATEWAY_USER"."ROLE"
(
    role_id smallint NOT NULL DEFAULT nextval('"SMS_GATEWAY_USER"."ROLE_role_id_seq"'::regclass),
    name character varying(15) COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp with time zone NOT NULL,
    CONSTRAINT "ROLE_pkey" PRIMARY KEY (role_id),
    CONSTRAINT "ROLE_name_key" UNIQUE (name)
)