FROM rust:1.95-bookworm AS builder

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --version 0.3.6 --locked

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY .env.example ./
COPY src ./src

RUN cp .env.example .env \
    && cargo leptos build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/Cargo.toml ./Cargo.toml
COPY --from=builder /app/.env.example ./.env.example
COPY --from=builder /app/target/release/pgtest002 ./pgtest002
COPY --from=builder /app/target/site ./target/site

ENV HOST=0.0.0.0
ENV PORT=8080
ENV DATABASE_URL=postgres://postgres:postgres@postgres:5432/pgtest002
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080
ENV LEPTOS_SITE_ROOT=target/site

EXPOSE 8080

CMD ["./pgtest002"]
