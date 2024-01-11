
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