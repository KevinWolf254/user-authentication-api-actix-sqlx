docker network create sms_gateway

docker run -d --name sms_gateway_db \
           -p 5432:5432 \
           --network sms_gateway \
           -e POSTGRES_PASSWORD=Pass12345 \
           -e POSTGRES_DB=SMS_GATEWAY \
           postgres:latest

docker run -p 5050:80 \
           --name sms_gateway_admin \
           --network sms_gateway \
           -e PGADMIN_DEFAULT_EMAIL=test@gmail.com \
           -e PGADMIN_DEFAULT_PASSWORD=Pass12345 \
           -d dpage/pgadmin4


CREATE SCHEMA "SMS_GATEWAY_USER"
    AUTHORIZATION postgres;

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA "SMS_GATEWAY_USER"
GRANT INSERT, SELECT, UPDATE ON TABLES TO pg_database_owner;

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA "SMS_GATEWAY_USER"
GRANT ALL ON SEQUENCES TO pg_database_owner;

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA "SMS_GATEWAY_USER"
GRANT EXECUTE ON FUNCTIONS TO pg_database_owner;

ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA "SMS_GATEWAY_USER"
GRANT USAGE ON TYPES TO pg_database_owner;