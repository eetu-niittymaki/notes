# syntax=docker/dockerfile:1

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Native dependencies required by Rust crates
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libwayland-dev \
    && rm -rf /var/lib/apt/lists/*


FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Build ONLY the server binary.
RUN cargo build --release --bin server


FROM debian:trixie-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libwayland-client0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /usr/local/bin/server

EXPOSE 8080

CMD ["/usr/local/bin/server"]
