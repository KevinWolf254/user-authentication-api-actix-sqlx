-- Add up migration script here
CREATE TABLE "SMS_GATEWAY_USER"."USER_ROLE"
(
    user_id integer NOT NULL,
    role_id smallint NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_user_role_id PRIMARY KEY (user_id, role_id),
    CONSTRAINT fk_user_role_to_user_id FOREIGN KEY (user_id) REFERENCES "SMS_GATEWAY_USER"."USER" (user_id),
    CONSTRAINT fk_user_role_to_role_id FOREIGN KEY (role_id) REFERENCES "SMS_GATEWAY_USER"."ROLE" (role_id)
);