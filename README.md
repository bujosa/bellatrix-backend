# bellatrix-backend

This api support bellatrix project

## Description

This project is a backend for bellatrix project. It is a REST API that allows you to manage users, chats and messages. It is written in Rust using the Actix framework and MongoDB as a database.

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- [Actix](https://actix.rs/docs/getting-started/)
- [MongoDB](https://docs.mongodb.com/manual/installation/)

## How to run

1. Install [Docker](https://docs.docker.com/install/)
2. Install [Docker Compose](https://docs.docker.com/compose/install/)
3. Run `docker-compose up -d` to start the server
4. Run `docker-compose down` to stop the server

Or you can run the server without docker

```rust
cargo run
```
