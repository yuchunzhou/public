A rust attribute macro used to make struct/union and its fields public

### How to use

add the dependency to your Cargo.toml

```toml
[dependencies]
public = { git = "https://github.com/yuchunzhou/public", branch = "main" }
```

then, mark the struct with `public` attribute macro

```rust
#[macro_use]
extern crate public;

#[public]
#[derive(Debug, Default)]
struct Foo {
    a: i8,
    b: char,
    c: String,
}
```

the struct `Foo` and its fields will be visible within the current crate, of cource, you can pass another scope argument to the `public` attribute macro, like the `pub` keyword in rust.
