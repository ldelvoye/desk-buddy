# Core

The `core` crate contains domain logic, application use cases, and adapter implementations for local persistence and scheduling.

## Current Architecture

The current module layout follows a hexagonal style with explicit adapter direction:

- `src/domain`: domain models and rules (for example `ReminderConfig`)
- `src/application`: use-case services (`HydrationService`, `SettingsService`)
- `src/ports`: trait contracts used by application services
- `src/adapters/inbound`: entrypoints that drive the application (`CoreFacade`, scheduler)
- `src/adapters/outbound`: implementations for external dependencies (SQLite, stdout event sink)
- `src/adapters/middleware`: placeholder for cross-cutting adapter concerns

## Logic Flow

### 1. Core initialization

`CoreFacade::new` performs startup wiring:

1. Open SQLite pool (`connect_sqlite`)
2. Run DB migrations (`run_migrations`)
3. Build outbound repositories and sinks
4. Build application services
5. Build inbound scheduler state

This creates a ready-to-use facade API for callers.

### 2. One-off hydration reminder

`trigger_hydration_reminder_once`:

1. Calls `HydrationService::trigger_reminder`
2. Service writes a reminder log via `HydrationLogRepository` port
3. Outbound SQLite adapter inserts into `hydration_reminder_logs`
4. Facade emits the message through `ReminderEventSink` (`StdoutReminderSink` currently prints it)

### 3. Persisted interval settings

`set_hydration_interval_minutes`:

1. Normalizes value through `ReminderConfig::new` (minimum `1`)
2. Persists through `SettingsRepository` port
3. Outbound SQLite adapter upserts into `hydration_settings`

`hydration_reminder_config`:

1. Loads stored interval through `SettingsRepository`
2. Falls back to default (`30`) when no value exists

### 4. Background reminder loop

`start_with_saved_hydration_config`:

1. Loads effective reminder config from settings
2. Calls `start(config)`

`start(config)` delegates to inbound `HydrationScheduler`, which:

1. Spawns a Tokio task loop
2. Sleeps for configured duration
3. Triggers reminder use case
4. Emits reminder event
5. Repeats until stopped

`stop()` signals shutdown and awaits task completion.

## Current SQLite Tables

`run_migrations` currently creates:

- `hydration_reminder_logs`
- `hydration_settings`

## Scope Notes

This crate currently implements the hydration slice and scheduler plumbing. Daily prompts, richer notification actions, and UI integration are future work.
