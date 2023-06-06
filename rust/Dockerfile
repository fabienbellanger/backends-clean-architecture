# TODO:
# - secure Dockerfile

FROM rust:latest as builder

LABEL maintainer="Fabien Bellanger <valentil@gmail.com>"

RUN apt-get update \
    && apt-get -y install ca-certificates cmake libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy
# ----
COPY ./domain/src domain/src
COPY ./domain/Cargo.toml domain/Cargo.toml

COPY ./infrastructure/src infrastructure/src
COPY ./infrastructure/assets infrastructure/assets
COPY ./infrastructure/migrations infrastructure/migrations
COPY ./infrastructure/templates infrastructure/templates
COPY ./infrastructure/Cargo.toml infrastructure/Cargo.toml
COPY ./infrastructure/sqlx-data.json infrastructure/sqlx-data.json

COPY ./shared/src shared/src
COPY ./shared/Cargo.toml shared/Cargo.toml

COPY ./.env.docker .env
COPY ./Cargo.toml Cargo.toml

# sqlx
# ----
ENV SQLX_OFFLINE true

# Build
# -----
ENV PKG_CONFIG_ALLOW_CROSS=1

# RUN cargo build --release
RUN cargo build

# =============================================================================

FROM gcr.io/distroless/cc AS runtime

WORKDIR /app

COPY --from=builder /app/.env .
COPY --from=builder /app/infrastructure/assets assets
COPY --from=builder /app/infrastructure/templates templates
# COPY --from=builder /app/target/release/clean-architecture-infrastructure .
COPY --from=builder /app/target/debug/clean-architecture-infrastructure .

ENTRYPOINT ["./clean-architecture-infrastructure"]
