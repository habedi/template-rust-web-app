# AGENTS.md

This file provides guidance to coding agents collaborating on this repository.

## Mission

This repository is a minimalistic project template for developing generic web applications in Rust, with
a Rust backend (`backend/`) and a SvelteKit frontend (`web/`).
Priorities, in order:

1. Correctness of the example code, since users copy it as a starting point.
2. Clean Architecture layering (domain, application, and infrastructure).
3. Minimalism, so the template stays easy to read and easy to strip down.
4. Clear, maintainable code.

## Core Rules

- Use English for code, comments, docs, and tests.
- Respect layer direction: `domain` depends on nothing, `application` depends on `domain`, and `infrastructure` depends
  on both. Do not import infrastructure types into domain or application code.
- Access external systems (PostgreSQL today) only through the trait interfaces in `backend/src/application/services/`.
- Prefer small, focused changes over large refactoring.
- Add comments only when they clarify non-obvious behavior.
- Write SQL keywords in lowercase in migrations and queries: `create table`, not `CREATE TABLE`.
- Keep the template generic. Resist adding a dependency that only one kind of application would need.

Quick examples:

- Good: add a new service trait in `application/services/` and implement it in `infrastructure/`.
- Bad: call `sqlx` directly from a use case in `application/use_cases/`.

## Writing Style

- Write in simple, plain English. Use short sentences and everyday words.
- Use Oxford commas in inline lists: "a, b, and c" not "a, b, c".
- Do not use em dashes. Restructure the sentence, or use a colon or semicolon instead.
- Avoid colorful adjectives and adverbs. Write "adjacency query" not "blazing adjacency query".
- Prefer noun phrases for checklist items over imperative verbs. Write "temp directory teardown" not "tear down the temp directory".
- Headings in Markdown files must be in title case: "Build from Source" not "Build from source". Minor words (a, an, the, and, but, or, for, in, on,
  at, to, by, of) stay lowercase unless they are the first word.
- Do not bold the lead-in of a list item. Write "Vector and set similarity: ..." not "**Vector and set similarity**: ...".
- Use sentence case for the lead-in of a list item. Write "Seed selection: ..." not "Seed Selection: ...". Proper nouns keep their capitals.
- Capitalize only the first part of a hyphenated compound: "Full-text Search" in a heading, "Breadth-first" at the start of a sentence, and
  "breadth-first search" elsewhere. Never write "Breadth-First".
- Start each sentence with a capital letter, capitalize proper nouns (Rust, PostgreSQL, SvelteKit, TypeScript), and leave common nouns lowercase
  in the middle of a sentence.
- Write correct and complete sentences.
- Avoid made-up words, abbreviations, and colons in the middle of sentences.
- Use participial phrases scarcely.

## Architecture Constraints

- The Rust toolchain is pinned to 1.97.1 (edition 2024) by `rust-toolchain.toml`; do not add version suffixes to cargo
  commands.
- The frontend targets the active Node.js LTS line, which is 24. TypeScript stays on the 5.x line because
  `svelte-check` and `typescript-eslint` both reject TypeScript 7.
- The workspace denies `clippy::unwrap_used` and `clippy::expect_used` in production code. Test code is exempt through
  the `cfg_attr` at the top of `backend/src/main.rs`.
- Queries use the runtime `sqlx::query_as` API, not the `sqlx::query!` macros, so the project builds without a database
  connection and carries no `.sqlx` metadata directory. Keep it that way unless the whole project moves to the macros.
- Rows are mapped through a private `ItemRow` type in `infrastructure/pg/`, so no `sqlx` derive appears on a domain
  entity.
- The backend applies its migrations on startup. A schema change means a new migration pair, never an edit to an
  existing one.
- Field-level validation belongs in the `validator` rules on the request body; invariants belong in the domain
  constructor (`ItemDraft::new`).
- Configuration comes from environment variables parsed in `backend/src/config.rs`; `make run-backend` copies
  `backend/.env.example` to `backend/.env` when missing, and `make run-web` does the same for `web/.env.example`
  (`VITE_API_URL`).
- Never commit real secrets. The `.env` files are gitignored, and example values belong in the `.env.example` files.

## Required Validation

Run these checks for any non-trivial backend change:

1. `make test` (unit tests; no containers needed)
2. `make lint` (clippy denies warnings, `unwrap_used`, and `expect_used`)
3. `make format`

Run these as well when the change touches SQL, migrations, or the repository layer:

1. `make docker-up` (starts PostgreSQL)
2. `make test-integration` (unit and integration tests against the live container)

For frontend changes, run `make lint-web`, `make check-web`, and `make test-web`.

`make check-all` runs every check that does not need containers.

## Review Guidelines (P0/P1 Focus)

Review output should be concise and only include critical issues.

- `P0`: must-fix defects (security flaw, data loss, architecture breakage).
- `P1`: high-priority defects (likely functional bug, missing validation, layering violation).

Do not include:

- style-only nitpicks,
- praise/summary of what is already good,
- exhaustive restatement of the patch.

Use this review format:

1. `Severity` (`P0`/`P1`)
2. `File:line`
3. `Issue`
4. `Why it matters`
5. `Minimal fix direction`

## Practical Notes for Agents

- Prefer targeted edits over broad mechanical rewrites.
- If you detect contradictory repository conventions, follow existing code and update docs accordingly.
- The `tmp/` directory is gitignored scratch space. Do not read from it or write to it as part of a change.

## Commit and PR Hygiene

- Keep commits scoped to one logical change.
- Follow the existing conventional commit style: `feat(items): ...`, `chore: ...`, `docs(readme): ...`.
- PR descriptions should include:
    1. behavioral change summary,
    2. tests added/updated,
    3. migrations added (or "no schema change"),
    4. docs updated (yes/no).
