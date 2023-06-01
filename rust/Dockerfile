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
# TODO: Improve copy
COPY ./domain domain
COPY ./infrastructure infrastructure
COPY ./shared shared
COPY ./.env.docker .env
COPY ./Cargo.toml Cargo.toml

# sqlx
# ----
# RUN cargo install sqlx-cli
# RUN cargo sqlx prepare -- --bin clean-architecture-infrastructure
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
CMD [""]
