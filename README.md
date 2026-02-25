## toml-comment

[![CI](https://github.com/z1xus/toml-comment/actions/workflows/ci.yml/badge.svg)](https://github.com/z1xus/toml-comment/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/toml-comment.svg)](https://crates.io/crates/toml-comment)
[![docs.rs](https://docs.rs/toml-comment/badge.svg)](https://docs.rs/toml-comment)

Derive macro that turns `///` doc comments and `Default` values into commented TOML.

### Why

The `toml` crate can't write comments. If your app ships a default config, you end up maintaining a hand-written TOML string that duplicates your `Default` impl, and the two inevitably drift apart.

[`toml_edit`](https://docs.rs/toml_edit) preserves comments but it's a round-trip parser for modifying existing files, not generating new ones. [`toml-scaffold`](https://crates.io/crates/toml-scaffold) can generate configs but depends on `schemars` and `JsonSchema`, which is a heavy dependency tree for what's a pretty simple problem.

The goal of this crate is to stay as small as possible. It reads your doc comments and defaults and produces commented TOML. Compile-time deps are `syn`/`quote`, runtime deps are `toml` + `serde`, and that's all there will ever be.

### Usage

```toml
[dependencies]
toml-comment = "0.2"
serde = { version = "1", features = ["derive"] }
```

```rust
use serde::Serialize;
use toml_comment::TomlComment;

#[derive(Serialize, TomlComment)]
struct ServerConfig {
    /// Port to listen on
    port: u16,
    /// Bind address
    host: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { port: 8080, host: "127.0.0.1".to_string() }
    }
}

std::fs::write("server.toml", ServerConfig::default_toml()).unwrap();
```

Output:

```toml
# Port to listen on
port = 8080
# Bind address
host = "127.0.0.1"
```

Nested structs become `[section]` headers automatically. `to_commented_toml()` serializes non-default values.

### Supported types

- Primitives (`bool`, integers, floats, `usize`, `isize`)
- `String`
- `Option<T>` -- omitted when `None`
- `Vec<T>` -- inline arrays
- Enums -- use `#[toml_comment(inline)]` on the field (the enum itself just needs `Serialize`)
- `HashMap<String, T>` / `BTreeMap<String, T>` -- leaf values become flat `key = value` pairs, struct values become inline tables
- Nested structs -- become `[section]` tables, must also derive `TomlComment`
- `#[toml_comment(inline)]` forces a struct field to serialize as an inline value

### How it works

The derive macro extracts `///` doc comments (rustc stores these as `#[doc = "..."]` attributes), classifies each field as a leaf or nested struct, and generates a `_render` method that serializes fields one by one through `toml::Value::try_from`.

The trait requires `Serialize + Default`. `default_toml()` calls `Self::default().to_commented_toml()`.
