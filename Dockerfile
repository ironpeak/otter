FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.toml
COPY ./src/ ./src/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY ./src/ ./src/
RUN cargo build --release --bin otter

FROM alpine:latest
RUN addgroup -S otter && adduser -S otter -G otter
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/otter /usr/local/bin/
USER otter
CMD ["/usr/local/bin/otter"]
