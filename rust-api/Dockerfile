FROM rust:1.83 as builder

WORKDIR /app
COPY Cargo.toml Config.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-api /usr/local/bin/rust-api
COPY Config.toml ./Config.toml

EXPOSE 8080
CMD ["rust-api"]

