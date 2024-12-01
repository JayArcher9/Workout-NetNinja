# Stage 1: Build
FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:buster-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/rust_facts_api /usr/local/bin/rust_facts_api
CMD ["rust_facts_api"]
EXPOSE 8080
