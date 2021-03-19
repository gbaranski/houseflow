FROM rust:latest

RUN cargo install cargo-watch

WORKDIR /app

COPY db db
COPY fulfillment fulfillment 

WORKDIR /app/fulfillment

CMD cargo watch -w /app/db/ -w /app/fulfillment -x run
