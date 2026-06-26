# implicitly

A Rust extension trait for wrapping values into `Option` and `Result` using method syntax.

## Motivation

Instead of writing constructors explicitly:

```rust
Some(value)
None::<T>
Ok(value)
Err(error)
```

You can use method syntax:

```rust
value.some()
T::none()
value.ok()
error.err()
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
implicitly = "0.1.0"
```

Then bring the traits into scope:

```rust
use implicitly::*;

let x = 42.some();           // Some(42)
let y = i32::none();         // None::<i32>
let z = 42.ok::<&str>();     // Ok(42)
let e = "oops".err::<i32>(); // Err("oops")
```

Use only what you need:

```rust
use implicitly::IntoOption;

let x = 42.some(); // Some(42)
```

## API

| Method | Equivalent |
|--------|------------|
| `value.some()` | `Some(value)` |
| `T::none()` | `None::<T>` |
| `value.ok::<E>()` | `Ok(value)` |
| `value.err::<A>()` | `Err(value)` |

## License

MIT
