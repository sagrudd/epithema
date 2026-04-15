FROM rust:1.85-bookworm AS builder

WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release -p emboss-cli

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /workspace/target/release/emboss-rs /usr/local/bin/emboss-rs

ENTRYPOINT ["/usr/local/bin/emboss-rs"]
CMD ["--help"]
