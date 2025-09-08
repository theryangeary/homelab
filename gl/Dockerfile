# build frontend
FROM node:24 as frontend
WORKDIR /app
COPY gl/ts .
RUN npm install
RUN npm run build

# use chef for faster rust builds/better caching
FROM lukemathwalker/cargo-chef:latest-rust-1.87 AS chef
WORKDIR /app

# generate chef plan
FROM chef AS planner

COPY gl/Cargo.toml gl/Cargo.lock ./
COPY gl/src ./src

RUN cargo chef prepare --recipe-path recipe.json

# build rust bins
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY gl/Cargo.toml gl/Cargo.lock ./
COPY gl/src ./src
COPY --from=frontend /app/dist ./ts/dist

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/grocery-list-backend ./grocery-list-backend

EXPOSE 3001

CMD ["./grocery-list-backend"]
