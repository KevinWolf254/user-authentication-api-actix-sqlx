-- Add up migration script here
CREATE TABLE "SMS_GATEWAY_USER"."USER"
(
    user_id serial NOT NULL,
    first_name character varying(150) NOT NULL,
    middle_name character varying(150),
    surname character varying(150) NOT NULL,
    email_address character varying(150) NOT NULL,
    mobile_number character varying(150),
    enabled boolean NOT NULL DEFAULT FALSE,
    email_confirmed boolean NOT NULL DEFAULT FALSE,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_user_id PRIMARY KEY (user_id),
    CONSTRAINT uq_user_email_address UNIQUE (email_address)
);