# use chef for faster rust builds/better caching
FROM lukemathwalker/cargo-chef:latest-rust-1.87 AS chef
WORKDIR /app

# generate chef plan
FROM chef AS planner

COPY src/rust/Cargo.toml src/rust/Cargo.lock ./
COPY src/rust/src ./src

RUN cargo chef prepare --recipe-path recipe.json

# build rust bins
FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY src/rust/Cargo.toml src/rust/Cargo.lock ./
COPY src/rust/src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/grocery-list-backend ./grocery-list-backend

EXPOSE 3001

CMD ["./grocery-list-backend"]