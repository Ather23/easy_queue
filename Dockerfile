FROM rust:latest AS builder


RUN USER=root cargo new --bin easy_queue
WORKDIR /easy_queue
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


RUN cargo build --release
RUN rm src/*.rs

COPY src ./src

RUN cargo build --release

FROM rust:latest

WORKDIR /app

COPY --from=builder /easy_queue/target/release/easy_queue  /app

ENV RUST_LOG=info

EXPOSE 3000

CMD ["./easy_queue"]