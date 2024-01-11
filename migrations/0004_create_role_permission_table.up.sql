-- Add up migration script here
CREATE TABLE "SMS_GATEWAY_USER"."ROLE_PERMISSION"
(
    role_id smallint NOT NULL,
    permission_id smallint NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_role_permission_id PRIMARY KEY (role_id, permission_id),
    CONSTRAINT fk_role_permission_role_id FOREIGN KEY (role_id) REFERENCES "SMS_GATEWAY_USER"."ROLE" (role_id),
    CONSTRAINT fk_role_permission_permission_id FOREIGN KEY (permission_id) REFERENCES "SMS_GATEWAY_USER"."PERMISSION" (permission_id)
);