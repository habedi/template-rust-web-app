## Backend

An axum service that exposes the example `items` resource over HTTP and stores it in PostgreSQL.

The code follows Clean Architecture layering:

- `src/domain/`: entities and domain errors, with no framework dependencies.
- `src/application/`: service traits (`services/`) and use cases (`use_cases/`).
- `src/infrastructure/`: the PostgreSQL implementation (`pg/`) and the HTTP layer (`web/`).

See [.env.example](.env.example) for the environment variables and [src/config.rs](src/config.rs) for their defaults.

### Run Locally

From the repository root:

```bash
make docker-up     # Start PostgreSQL
make run-backend   # Run the service on http://localhost:8080
```

The service applies the migrations in [migrations](migrations) on startup.

### Tests

```bash
make test              # Unit tests, no containers needed
make test-integration  # Unit and integration tests, needs `make docker-up`
```

Unit tests mock the service traits with `mockall`.
Integration tests sit behind the `integration-tests` feature so a plain `cargo test` never needs a database.

### Migrations

Migrations run on startup, so this is only needed when managing them by hand with
[sqlx-cli](https://crates.io/crates/sqlx-cli):

```bash
make db-migrate  # Apply the pending migrations
make db-revert   # Revert the most recent migration
```

A schema change means a new timestamp-prefixed `.up.sql` and `.down.sql` pair, never an edit to an existing one.

### Queries

Queries use the runtime `sqlx::query_as` API, so the crate builds without a database connection and without checked-in query metadata.
Rows are mapped through a private `ItemRow` type in [src/infrastructure/pg](src/infrastructure/pg) to keep the `sqlx` derive out of the domain layer.

To get compile-time query verification instead, switch those calls to `sqlx::query_as!`, set `DATABASE_URL` at build time, and commit the output
of `cargo sqlx prepare`.
