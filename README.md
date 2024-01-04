
# Rust SQLx Application

## Overview

This Rust application demonstrates the usage of the SQLx library to interact with a PostgreSQL database. The application models a basic permission system with tables for permissions, roles, role_permissions (many-to-many relationship), and users (one-to-many relationship with roles).

## Prerequisites

- Rust
- PostgreSQL

## Setup

1. Clone the repository:

```bash
  git clone https://github.com/your_username/your_project.git
```
2. Navigate to the project directory:

```bash
  cd project_name
```

3. Install dependencies:

```bash
  cargo build
```

4. Set up the PostgreSQL database:

```bash
docker network create sms_gateway
```

```bash
docker run -d --name sms_gateway_db \
           -p 5432:5432 \
           --network sms_gateway \
           -e POSTGRES_PASSWORD=Pass12345 \
           -e POSTGRES_DB=SMS_GATEWAY \
           postgres:latest
```
5. Set up Admin4 to access postgresql

```bash
docker run -p 5050:80 \
           --name sms_gateway_admin \
           --network sms_gateway \
           -e PGADMIN_DEFAULT_EMAIL=test@gmail.com \
           -e PGADMIN_DEFAULT_PASSWORD=Pass12345 \
           -d dpage/pgadmin4
```
Login to Admin4 and create a schema `SMS_GATEWAY_USE`

6. Install sqlx

```bash
cargo install sqlx-cli
```

7. Run migrations

```bash
sqlx migrate run
```

To revert run

```bash
sqlx migrate revert
```
8. Run the project

```bash
  cargo run
```

## Contributing

Feel free to contribute to this project by opening issues or submitting pull requests. Any feedback or improvements are welcome!

## License

This project is licensed under the MIT License - see the LICENSE file for details.