-- Add up migration script here
CREATE TABLE "SMS_GATEWAY_USER"."PERMISSION"
(
    permission_id smallserial NOT NULL,
    name character varying(150) NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_permission_id PRIMARY KEY (permission_id),
    CONSTRAINT uq_permission_name UNIQUE (name)
);