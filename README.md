# HTTP Codex

HTTP codex is a library for managing HTTP codes safely. This library was written to be used in my projects, primarily
becaus I didn't find any existing one that covers my needs.
I'm publishig it here in case somebody else is interested.

# The enums

## HTTP Code

The first enum is `HttpCode` which contains every known HTTP code. You can convert integers and integers inside
an `Option` to this enum. If a code is unknown, an `HttpCode::Unknown` variant is created and if an `Option::None` is
given, a `HttpCode::None` is created.

```rust
use http_codex::HttpCode;

let code = HttpCode::from(307);
assert!(matches!(code, HttpCode::TemporaryRedirect));

let code = HttpCode::from(Some(410));
assert!(matches!(code, HttpCode::Gone));
```

## HTTP code class

The second enum is `HttpCodeClass` containing every class so you don't have to check if the numerical value of a code is
in a range. You can easily convert an `HttpCode` to an `HttpCodeClass`.

```rust
use http_codex::HttpCode;
use http_codex::HttpCodeClass;

let code = HttpCode::from(302);
let class = HttpCodeClass::from(code);
assert!(matches!(class, HttpCodeClass::Redirection));
```
