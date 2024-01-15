
Create a database.
Update the DATABASE_URL in the .env file with your database connection details.

1. Install sqlx

```bash
cargo install sqlx-cli
```

2. Create the database 

```bash
sqlx database create
```

3. Add migration with revert

```bash
sqlx migrate add -r create_permission_table
sqlx migrate add -r create_role_table
sqlx migrate add -r create_role_permission_table
sqlx migrate add -r create_user_table
```

4. Add script to create tables

Create PERMISSION table: 

```bash
CREATE TABLE "SMS_GATEWAY_USER"."PERMISSION"
(
    permission_id smallserial NOT NULL,
    name character varying(150) NOT NULL,
    created_at timestamp with time zone NOT NULL,
    PRIMARY KEY (permission_id),
    UNIQUE (name)
);
```

Create ROLE table: 

```bash
CREATE TABLE "SMS_GATEWAY_USER"."ROLE"
(
    role_id smallserial NOT NULL,
    name character varying(150) NOT NULL,
    created_at timestamp with time zone NOT NULL,
    CONSTRAINT pk_role_id PRIMARY KEY (role_id),
    CONSTRAINT uq_role_name UNIQUE (name)
);
```
Create the ROLE_PERMISSION join table:

```bash
CREATE TABLE "SMS_GATEWAY_USER"."ROLE_PERMISSION"
(
    role_id smallint NOT NULL,
    permission_id smallint NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_role_permission_id PRIMARY KEY (role_id, permission_id),
    CONSTRAINT fk_role_permission_role_id FOREIGN KEY (role_id) REFERENCES "SMS_GATEWAY_USER"."ROLE" (role_id),
    CONSTRAINT fk_role_permission_permission_id FOREIGN KEY (permission_id) REFERENCES "SMS_GATEWAY_USER"."PERMISSION" (permission_id)
);
```

Create the USER table:

```bash
CREATE TABLE "SMS_GATEWAY_USER"."USER"
(
    user_id smallint NOT NULL,
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
```

5. Add script to revert tables

```bash
DROP TABLE IF EXISTS "SMS_GATEWAY_USER"."PERMISSION" RESTRICT;
```

```bash
DROP TABLE IF EXISTS "SMS_GATEWAY_USER"."ROLE" RESTRICT;
```

6. Run migrations

```bash
sqlx migrate run
```

## NB: To revert migrations, run:

```bash
sqlx migrate revert
```