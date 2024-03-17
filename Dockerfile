# Build Stage
FROM rust:1.76-bullseye as builder
WORKDIR /app
COPY . .
RUN cargo clean
RUN cargo build --release

# Production Stage
FROM debian:bullseye-slim
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/rust-currency-converter .
CMD ["./rust-currency-converter"]