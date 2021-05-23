A macro used to make struct/union and its fields public

### How to use

add the dependency to your Cargo.toml

```toml
[dependencies]
public = { git = "https://github.com/yuchunzhou/public", branch = "main" }
```

then, mark the struct with `make_public` macro

```rust
#[macro_use]
extern crate public;

#[make_public]
#[derive(Debug, Default)]
struct Foo {
    a: i8,
    b: char,
    c: String,
}
```

the struct `Foo` and its fields will be public, also you can limit the items' visibility within a
given scope likes the `pub` restrictions in Rust.
