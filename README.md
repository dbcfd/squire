# Squire Test

API demonstrating users and favorite cities

## Setup

* Install [postgres](https://www.postgresql.org/download/)
* Setup the database

    sudo -u postgres psql -c "CREATE ROLE squireuser WITH LOGIN SUPERUSER PASSWORD 'password';"
    createdb -h postgres -p 5432 -U squireuser squire

* Install [sqitch](https://sqitch.org/download/)
* Run migrations

    sqitch deploy db:pg:squire

* Build the application

    cargo build --release

* Run the application

    cargo run