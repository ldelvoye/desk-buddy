# Core Review Order

Use this order to get familiar with the current architecture and Rust patterns in this crate.

## 1) Crate surface (5 min)

1. `core/src/lib.rs`
2. `core/src/main.rs`

## 2) Architecture map (10 min)

3. `core/README.md`

## 3) Bootstrap + runtime lifecycle (15 min)

4. `core/src/bootstrap/mod.rs`
5. `core/src/bootstrap/core_builder.rs`
6. `core/src/bootstrap/core_runtime.rs`

## 4) API surface (10 min)

7. `core/src/api/mod.rs`
8. `core/src/api/core_api.rs`

## 5) Use-case layer (10 min)

9. `core/src/application/mod.rs`
10. `core/src/application/hydration_service.rs`
11. `core/src/application/settings_service.rs`

## 6) Domain + invariants (8 min)

12. `core/src/domain/mod.rs`
13. `core/src/domain/hydration.rs`

## 7) Ports/contracts (8 min)

14. `core/src/ports/mod.rs`
15. `core/src/ports/hydration_log_repository.rs`
16. `core/src/ports/settings_repository.rs`
17. `core/src/ports/reminder_event_sink.rs`

## 8) Outbound adapters (12 min)

18. `core/src/adapters/outbound/mod.rs`
19. `core/src/adapters/outbound/sqlite/mod.rs`
20. `core/src/adapters/outbound/sqlite/connection.rs`
21. `core/src/adapters/outbound/sqlite/migrations.rs`
22. `core/src/adapters/outbound/sqlite/hydration_log_repository.rs`
23. `core/src/adapters/outbound/sqlite/settings_repository.rs`
24. `core/src/adapters/outbound/events/stdout_sink.rs`

## 9) Inbound adapter (scheduler) (10 min)

25. `core/src/adapters/inbound/mod.rs`
26. `core/src/adapters/inbound/scheduler/mod.rs`
27. `core/src/adapters/inbound/scheduler/hydration_scheduler.rs`

## 10) Error model (5 min)

28. `core/src/error.rs`

---

## Rust Concepts To Watch For

- `Arc<dyn Trait>` dependency injection style
- `Result<T, E>` and `?` error propagation
- `#[tokio::main]` and async function boundaries
- `Mutex<T>` + `MutexGuard` in async contexts
- `#[cfg(test)]` colocated tests
- `pub use` exports in `mod.rs` and `lib.rs`
