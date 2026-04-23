# PATH A: host `cargo leptos build --release` succeeded; runtime image copies pre-built artifacts.
# Self-check: host build succeeded, so PATH A is used.
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 libpq5 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home --home-dir /app --shell /usr/sbin/nologin appuser

WORKDIR /app

COPY target/release/pgtest002 /usr/local/bin/pgtest002
COPY --chown=appuser:appuser target/site ./target/site

ENV HOST=0.0.0.0
ENV PORT=8080
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080
ENV LEPTOS_SITE_ROOT=target/site
ENV RUST_LOG=info

USER appuser

EXPOSE 8080

CMD ["/usr/local/bin/pgtest002"]
