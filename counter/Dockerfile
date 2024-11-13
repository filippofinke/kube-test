# Build stage

FROM rust:1.76 AS builder

WORKDIR /app

COPY . .

RUN rustup target add aarch64-unknown-linux-musl
RUN cargo build --release --target=aarch64-unknown-linux-musl

# This is the final image

FROM alpine:latest

COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/counter /usr/local/bin/counter


ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
RUN chmod +x /usr/local/bin/counter

CMD ["counter"]