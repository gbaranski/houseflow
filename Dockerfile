FROM rust:latest as builder
WORKDIR /usr/src/houseflow-server
COPY . .
RUN cargo install --path server

FROM debian:latest
COPY --from=builder /usr/local/cargo/bin/houseflow-server /usr/local/bin/houseflow-server
ENV HOUSEFLOW_SERVER_CONFIG=/etc/houseflow/server.toml
CMD ["houseflow-server"]
