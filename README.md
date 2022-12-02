# Squire Test

API demonstrating users and favorite cities

## Setup

This application uses [postgres](https://www.postgresql.org/download/) as the backend.

To setup the database

    sudo -u postgres psql -c "CREATE ROLE squireuser WITH LOGIN SUPERUSER PASSWORD 'password';"
    createdb -h localhost -p 5432 -U squireuser squire

[sqitch](https://sqitch.org/download/) is then used for data migrations

    sqitch deploy local

At this point, the application can be built and run

    cargo build --release
    cargo run
    