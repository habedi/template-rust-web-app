## Frontend

A SvelteKit application built with Svelte 5, Tailwind CSS, and TypeScript.
It renders the example `items` resource served by the backend.

The build uses `@sveltejs/adapter-static`, so the output is a set of static files served by nginx in the container.
Server-side rendering is off, and the browser talks to the backend directly.

Every dependency tracks its latest release except TypeScript, which stays on the 5.x line because `svelte-check` and
`typescript-eslint` both refuse to run against TypeScript 7.

### Layout

- `src/routes/`: pages and their `load` functions.
- `src/lib/components/`: components, with the small generic ones under `atoms/`.
- `src/lib/services/api/`: the API client.

### Run Locally

From the repository root:

```bash
make run-web  # Serve the app on http://localhost:5173
```

The dev server reads `VITE_API_URL` from `.env`, which `make run-web` creates from [.env.example](.env.example) when it
is missing. The backend has to be running for the pages to load data.

### Checks

```bash
make lint-web   # Prettier and ESLint
make check-web  # svelte-check
make test-web   # Vitest
make build-web  # Production build
```
