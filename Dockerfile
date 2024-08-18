FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

COPY --from=0 /app/target/release/easy_queue .

EXPOSE 8080

CMD ["./easy_queue"]