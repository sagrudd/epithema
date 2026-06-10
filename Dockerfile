ARG EPITHEMA_VERSION=dev

FROM rust:1.85-bookworm AS builder

ARG EPITHEMA_VERSION

WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release --locked -p epithema-cli

FROM debian:bookworm-slim AS runtime

ARG EPITHEMA_VERSION

LABEL org.opencontainers.image.title="epithema" \
      org.opencontainers.image.description="Linux-first Epithema single-binary container image" \
      org.opencontainers.image.version="${EPITHEMA_VERSION}" \
      org.opencontainers.image.source="https://github.com/sagrudd/epithema" \
      org.opencontainers.image.licenses="MPL-2.0"

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && groupadd --system emboss \
    && useradd --system --gid emboss --create-home emboss \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /workspace/target/release/epithema /usr/local/bin/epithema

ENV EPITHEMA_VERSION=${EPITHEMA_VERSION}

USER emboss

ENTRYPOINT ["/usr/local/bin/epithema"]
CMD ["--help"]
