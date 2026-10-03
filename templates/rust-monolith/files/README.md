# {{name}}

    docker compose up -d      # Postgres on :5432
    cp .env.example .env
    cargo run                 # migrations run on startup

    curl localhost:3000/health
    curl -X POST localhost:3000/users \
      -H 'content-type: application/json' \
      -d '{"email":"a@b.c","name":"Alice"}'

## Layout

| Path | Holds |
|---|---|
| `config.rs` | settings from the environment |
| `telemetry.rs` | tracing setup, levels via `RUST_LOG` |
| `error.rs` | `AppError` — every handler returns it |
| `db.rs` | connection pool, runs `migrations/` on startup |
| `migrations/` | SQL, applied in order, never edited after release |

SQL is written with `sqlx::query_as` rather than the `query_as!` macro, so the
project builds without a live database. Once the schema settles, switch to the
macro and `cargo sqlx prepare` to have the compiler check your SQL.
