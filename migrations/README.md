
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
```

4 Add script to create tables

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

5. Add script to revert tables

```bash
DROP TABLE IF EXISTS "SMS_GATEWAY_USER"."PERMISSION" RESTRICT;
```

6. Run migrations

```bash
sqlx migrate run
```

## NB: To revert migrations, run:

```bash
sqlx migrate revert
```