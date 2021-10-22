FROM rust:latest as chef

RUN cargo install cargo-chef 
WORKDIR app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin houseflow-server

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/houseflow-server /usr/local/bin
ENV HOUSEFLOW_SERVER_CONFIG=/etc/houseflow/server.toml
ENTRYPOINT ["/usr/local/bin/houseflow-server"]
