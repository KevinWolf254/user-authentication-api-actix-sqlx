-- Add migration script here
CREATE TABLE "SMS_GATEWAY_USER"."USER_CODE"
(
    user_code_id serial NOT NULL,
    code integer NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_user_code_id PRIMARY KEY (user_code_id),
    CONSTRAINT uq_user_code_user_id UNIQUE (user_id),
    CONSTRAINT fk_user_code_user_id FOREIGN KEY (user_id) REFERENCES "SMS_GATEWAY_USER"."USER" (user_id)
);