FROM rust:latest

RUN cargo install cargo-watch

WORKDIR /app

COPY db db
COPY fulfillment fulfillment 
RUN ls

WORKDIR /app/fulfillment

CMD ["cargo", "watch", "-x", "'run'"]
