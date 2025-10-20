# Todo Web Application in Rust with Rocket

A demonstration web application built with the Rust programming language and the Rocket web framework.

## Features

- User registration
- Login
- Authorization
- JWT-protected API endpoints
- Todo Management - Create, list, view, and update todo items
- PostgreSQL database with SQL migrations

## Packages used

1. [rocket](https://crates.io/crates/rocket) - for routing.
2. [argon2](https://crates.io/crates/argon2) - to hash password.
3. [dotenv](https://crates.io/crates/dotenv) - to read dotenv file.
4. [envy](https://crates.io/crates/envy) - to parse envs file into struct.
5. [jsonwebtoken](https://crates.io/crates/jsonwebtoken) - to generate & verify JWT.
6. [sqlx](https://crates.io/crates/sqlx) - SQL migrations & queries.
