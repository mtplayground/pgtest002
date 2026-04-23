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
- `docker-compose.yml`: starts the application and PostgreSQL together, with a named `postgres_data` volume for database persistence and environment-variable overrides for ports and database settings.

Start the full stack with:

```bash
docker compose up -d --build
```

This publishes:

- App: `http://localhost:${APP_PORT:-8080}`
- Postgres: `localhost:${POSTGRES_PORT:-5432}`

Verify the stack after startup:

```bash
docker compose ps
docker compose logs --no-color app postgres
curl -I http://localhost:8080
```

The app container is healthy when the `app` service stays running and the HTTP endpoint responds. The Postgres container is healthy when `docker compose ps` reports the `postgres` health check as `healthy`.

## Docker Overrides

The Compose file uses shell-style defaults, so you can override values inline or through a local `.env` file that Docker Compose reads automatically.

Examples:

```bash
APP_PORT=8081 POSTGRES_PORT=55432 docker compose up -d --build
```

```bash
POSTGRES_DB=pgtest002_prod \
POSTGRES_USER=pgtest002 \
POSTGRES_PASSWORD=change-me \
docker compose up -d --build
```

Supported overrides:

| Variable | Default | Purpose |
| --- | --- | --- |
| `APP_PORT` | `8080` | Host port mapped to the Leptos/Axum container port `8080`. |
| `POSTGRES_PORT` | `5432` | Host port mapped to the Postgres container port `5432`. |
| `HOST` | `0.0.0.0` | Bind host inside the app container. |
| `PORT` | `8080` | Bind port inside the app container. |
| `LEPTOS_SITE_ADDR` | `0.0.0.0:8080` | Leptos SSR/static-file site address inside the app container. |
| `LEPTOS_SITE_ROOT` | `target/site` | Location of generated site assets inside the app container. |
| `POSTGRES_DB` | `pgtest002` | Database name created by the Postgres container. |
| `POSTGRES_USER` | `postgres` | Database user created by the Postgres container. |
| `POSTGRES_PASSWORD` | `postgres` | Database password used by Postgres and the app connection string. |
| `DATABASE_URL` | Derived from the Postgres defaults | Full override for the app database connection string. Use this only if you need the app to connect somewhere other than the bundled `postgres` service. |

Stop the stack with:

```bash
docker compose down
```

Remove the Postgres volume as well:

```bash
docker compose down --volumes
```

## Troubleshooting

- `docker: command not found`: install Docker Engine and the Docker Compose plugin, then rerun `docker compose version`.
- Port already in use: override `APP_PORT` or `POSTGRES_PORT`, for example `APP_PORT=8081 POSTGRES_PORT=55432 docker compose up -d --build`.
- App cannot connect to Postgres: if you override `POSTGRES_DB`, `POSTGRES_USER`, or `POSTGRES_PASSWORD`, keep `DATABASE_URL` aligned or let Compose derive it from the same defaults.
- Stale image or dependency cache: rerun `docker compose build --no-cache app` and then `docker compose up -d`.
- Stale database state: use `docker compose down --volumes` to recreate the named Postgres volume from scratch.
- Need startup logs while debugging: run `docker compose logs -f app postgres`.
