ARG EMBOSS_RS_VERSION=dev

FROM rust:1.85-bookworm AS builder

ARG EMBOSS_RS_VERSION

WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release --locked -p emboss-cli

FROM debian:bookworm-slim AS runtime

ARG EMBOSS_RS_VERSION

LABEL org.opencontainers.image.title="emboss-rs" \
      org.opencontainers.image.description="Linux-first EMBOSS-RS single-binary container image" \
      org.opencontainers.image.version="${EMBOSS_RS_VERSION}" \
      org.opencontainers.image.source="https://github.com/sagrudd/emboss-rs" \
      org.opencontainers.image.licenses="MPL-2.0"

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && groupadd --system emboss \
    && useradd --system --gid emboss --create-home emboss \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /workspace/target/release/emboss-rs /usr/local/bin/emboss-rs

ENV EMBOSS_RS_VERSION=${EMBOSS_RS_VERSION}

USER emboss

ENTRYPOINT ["/usr/local/bin/emboss-rs"]
CMD ["--help"]
