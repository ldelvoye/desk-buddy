# Desk Buddy

Desk Buddy is a lightweight desktop companion project focused on architecture-first development in Rust.

## Scope

- Windows-first desktop app
- Fully local execution (no deployed backend)
- Core architecture designed to support future backend migration
- Initial MVP features:
  - hydration reminders
  - daily prompt and journal entry flow

Detailed product scope and constraints live in `docs/REQUIREMENTS.md`.

## Repository Overview

- `core/`: Rust core logic (domain, use cases, ports, inbound/outbound adapters)
- `docs/REQUIREMENTS.md`: project goals, UX constraints, and roadmap
- `docs/TODOS.md`: prioritized implementation backlog

## Runtime Configuration

- Bootstrap config is resolved with precedence: env vars -> `config.toml` -> built-in defaults.
- Current bootstrap override: `DESK_BUDDY_DATABASE_URL`.
- Runtime user settings (for example hydration interval) are persisted in SQLite and changed via core API.

## Core Documentation

For architecture and runtime logic flow of the current core implementation, see:

- [`core/README.md`](core/README.md)
