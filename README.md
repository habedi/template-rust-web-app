## A Template for Rust Web Applications

<div align="center">
  <picture>
    <img alt="template-rust-web-app logo" src="docs/assets/logos/rustacean-flat-happy.svg" height="40%" width="40%">
  </picture>
<br>

[![Tests](https://img.shields.io/github/actions/workflow/status/habedi/template-rust-web-app/tests.yml?label=tests&style=flat&labelColor=282c34&color=4caf50&logo=github)](https://github.com/habedi/template-rust-web-app/actions/workflows/tests.yml)
[![Lints](https://img.shields.io/github/actions/workflow/status/habedi/template-rust-web-app/lints.yml?label=lints&style=flat&labelColor=282c34&color=4caf50&logo=github)](https://github.com/habedi/template-rust-web-app/actions/workflows/lints.yml)
[![Docker Images](https://img.shields.io/github/actions/workflow/status/habedi/template-rust-web-app/docker.yml?label=images&style=flat&labelColor=282c34&color=4caf50&logo=docker)](https://github.com/habedi/template-rust-web-app/actions/workflows/docker.yml)
[![Docs](https://img.shields.io/badge/docs-latest-007ec6?style=flat&labelColor=282c34&logo=readthedocs)](docs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-007ec6?style=flat&labelColor=282c34&logo=open-source-initiative)](https://github.com/habedi/template-rust-web-app)

</div>

---

This is an opinionated template with a minimalistic structure to make it easier to develop generic web applications in Rust. It ships a small backend
written with Axum and sqlx, a SvelteKit frontend, and the tooling to build, test, and run both.
I share it here in case it might be useful to others.

### Features

- Minimalistic project structure with a clear separation between the domain, application, and infrastructure layers
- A backend built with Axum, sqlx, and PostgreSQL, including graceful shutdown, structured logging, and CORS
- A SvelteKit frontend built with Svelte 5, Tailwind CSS, and TypeScript
- Unit tests that mock the service traits, plus opt-in integration tests that run against a real database
- Docker Compose setup for PostgreSQL, the backend, and the frontend
- Makefile for managing common tasks such as formatting, testing, linting, and building
- Pre-configured GitHub Actions for tests, lints, and image builds

### Repository Layout

| Path                          | Contents                                                   |
|-------------------------------|------------------------------------------------------------|
| `backend/`                    | Rust backend: Axum, sqlx, and PostgreSQL                   |
| `backend/src/domain/`         | Entities and domain errors, with no framework dependencies |
| `backend/src/application/`    | Service traits and use cases                               |
| `backend/src/infrastructure/` | PostgreSQL, HTTP routes, middleware, and error mapping     |
| `backend/migrations/`         | sqlx migrations as `.up.sql` and `.down.sql` pairs         |
| `web/`                        | SvelteKit frontend                                         |
| `docker-compose.yaml`         | `db`, `backend`, and `web` services                        |
| `Makefile`                    | Build, test, run, etc. commands                            |

### Getting Started

If you use Nix, `nix develop` (or `make shell`) gives you the pinned Rust toolchain, Node.js, sqlx-cli, and `psql` in one shell, and it sets
`DATABASE_URL` to match the database container. Otherwise install the tooling with `make install-deps`.

```bash
# Install the development tooling (sqlx-cli, cargo-audit, and the frontend packages)
make install-deps

# Start PostgreSQL
make docker-up

# Run the backend on http://localhost:8080
make run-backend

# In a second terminal, run the frontend on http://localhost:5173
make run-web
```

The backend applies its migrations on startup, so there is no separate migration step for local development. Use `make db-migrate` and
`make db-revert` when you want to manage them by hand.

To run the whole stack in containers instead, use `make docker-run` and open http://localhost.

Run `make help` to see every available target.

### Development

```bash
make check-all         # Lints and tests for both the backend and the frontend
make test              # Backend unit tests, no containers needed
make test-integration  # Backend unit and integration tests, needs `make docker-up`
make format            # Format the Rust code
make lint              # Run clippy with warnings denied
```

The example API is available under `/api/v1/items`, and `/health` reports liveness.

| Method   | Path                 | Description         |
|----------|----------------------|---------------------|
| `GET`    | `/health`            | Liveness probe      |
| `GET`    | `/api/v1/items`      | List the items      |
| `POST`   | `/api/v1/items`      | Create an item      |
| `GET`    | `/api/v1/items/{id}` | Fetch a single item |
| `PUT`    | `/api/v1/items/{id}` | Replace an item     |
| `DELETE` | `/api/v1/items/{id}` | Delete an item      |

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to make a contribution.

### License

This project is licensed under either of these:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
