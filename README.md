# pgtest002

`pgtest002` is a Leptos + Axum TodoMVC application backed by PostgreSQL.

## Requirements

- Rust with the `wasm32-unknown-unknown` target
- `cargo-leptos`
- Docker and Docker Compose for the container workflow

Install the local development tools:

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --version 0.3.6 --locked
```

## Environment Variables

Create a local `.env` file before running the app directly:

```bash
cp .env.example .env
```

The application reads the following variables:

| Variable | Required | Description | Example |
| --- | --- | --- | --- |
| `HOST` | Yes | Host interface for the Axum server bind address. | `0.0.0.0` |
| `PORT` | Yes | Port for the Axum server bind address. | `8080` |
| `DATABASE_URL` | Yes | PostgreSQL connection string used by the server. | `postgres://postgres:postgres@localhost:5432/pgtest002` |
| `LEPTOS_SITE_ADDR` | Yes | Site address exposed to Leptos for SSR/static file handling. | `0.0.0.0:8080` |
| `LEPTOS_SITE_ROOT` | Yes | Directory that contains the generated Leptos site assets. | `target/site` |

The checked-in [.env.example](.env.example) contains a working local baseline.

## Local Development

1. Start PostgreSQL locally.
2. Copy `.env.example` to `.env`.
3. Run the Leptos watcher:

```bash
cargo leptos watch
```

The application listens on `0.0.0.0:8080`. In the current config, Leptos writes generated assets to `target/site`.

If you want to verify the SSR binary without the watcher, run:

```bash
cargo build --release
```

## Docker Workflow

The repository includes:

- `Dockerfile`: a multi-stage container build that uses `cargo-leptos` in the builder stage and copies the compiled binary plus `target/site` into a slim runtime image.
- `docker-compose.yml`: starts the application and PostgreSQL together, with a named `postgres_data` volume for database persistence.

Start the full stack with:

```bash
docker compose up --build
```

This publishes:

- App: `http://localhost:8080`
- Postgres: `localhost:5432`

Stop the stack with:

```bash
docker compose down
```

Remove the Postgres volume as well:

```bash
docker compose down --volumes
```
