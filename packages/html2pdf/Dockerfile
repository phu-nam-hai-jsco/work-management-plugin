# syntax=docker/dockerfile:1
FROM rust:1-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked \
    && cp target/release/html2pdf /tmp/html2pdf-x86_64-unknown-linux-gnu

FROM scratch AS export
COPY --from=builder /tmp/html2pdf-x86_64-unknown-linux-gnu /html2pdf-x86_64-unknown-linux-gnu
