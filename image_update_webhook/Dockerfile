# use chef for faster rust builds/better caching
FROM lukemathwalker/cargo-chef:latest-rust-1.88 AS chef
WORKDIR /app

# generate chef plan
FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo chef prepare --recipe-path recipe.json

# build rust bins
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/image-update-webhook /image_update_webhook

EXPOSE 3000

ENTRYPOINT ["/image_update_webhook"]

