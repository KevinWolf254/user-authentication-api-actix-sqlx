-- Add migration script here
CREATE TABLE "SMS_GATEWAY_USER"."ROLE"
(
    role_id smallserial NOT NULL,
    name character varying(150) NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_role_id PRIMARY KEY (role_id),
    CONSTRAINT uq_role_name UNIQUE (name)
);