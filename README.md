A crate to export struct fields to be public 
### How to use it?
```rust
#[macro_use]
extern crate public;

#[make_public(crate)]
#[derive(Debug, Default)]
struct Foo {
    a: i8,
    b: char,
    c: String,
}
```
the struct `Foo` and its fields will be accessible in current crate globally.