# `rustdoc` and the Documentation Ecosystem

- Documentation can be marked with `///` comments, see example
- Documentation can be generated with `cargo doc --open`

```rust
/// This is a comment that will be auto-generated.
fn main () {
  foo();
}

fn foo () {
  println!("Hello, World!");
}
```
