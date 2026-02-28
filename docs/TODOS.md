# TODOs

## Database hardening

- [x] Define a stable production DB path policy (Windows app-data location) instead of relying on relative paths.
- [x] Ensure DB parent directories are created before SQLite connect in packaged app mode.
- [x] Add SQLite startup pragmas policy (WAL mode, busy timeout, foreign_keys).
- [x] Replace ad-hoc schema setup with versioned migrations strategy.

## Settings model and schema

- [x] Decide and document the expected shape of `hydration_settings`:
  - singleton row (`id = 1`) with one column per setting, or
  - key-value settings table for extensibility.
- [x] Add bootstrap behavior for missing settings row if we want a non-empty default row visible in DB tools.
- [ ] Add a migration path for future settings fields without breaking existing local DBs.

## Core feature data model

- [ ] Design and add tables for daily prompt and journal entries.
- [ ] Define retention/query requirements for hydration logs (indexing, paging, pruning policy).

## Runtime and operational behavior

- [ ] Add graceful shutdown and startup behavior expectations for scheduler/runtime.
- [ ] Add integration checks to verify persistence behavior across app restarts.
- [ ] Reconfigure or restart scheduler when `set_hydration_interval_minutes` changes interval mid-runtime so the new value applies immediately.
