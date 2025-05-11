# Dockerfile

FROM rust:1.86-slim-bookworm AS builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/hello-fly /usr/local/bin/hello-fly

ENV PORT=8080
EXPOSE 8080

CMD ["hello-fly"]