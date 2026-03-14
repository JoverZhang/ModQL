# Internal Module `service::handler`

[Surface view](module.service.handler.md)

Request handlers for the service layer.

## Sub-modules

| Module | Summary | Surface |
|---|---|---|
| [`auth`](module.service.handler.auth.internal.md) | Authentication and authorization handlers. | [surface](module.service.handler.auth.md) |

## Functions

```rust
/// Dispatch a request by name and return the response body.
pub fn dispatch(name: &str) -> String;

```

---

## Functions (private)

```rust
/// Log a handler event for auditing.
pub(in ::handler) fn log_event(event: &str);

```

