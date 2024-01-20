-- Add up migration script here
CREATE TABLE "SMS_GATEWAY_USER"."USER_CREDENTIAL"
(
    user_credential_id serial NOT NULL,
    username character varying(150) NOT NULL,
    password character varying(255) NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_user_credential_id PRIMARY KEY (user_credential_id),
    CONSTRAINT uq_user_credential_user_id UNIQUE (user_id),
    CONSTRAINT uq_username UNIQUE (username),
    CONSTRAINT fk_user_credential_user_id FOREIGN KEY (user_id) REFERENCES "SMS_GATEWAY_USER"."USER" (user_id)
);