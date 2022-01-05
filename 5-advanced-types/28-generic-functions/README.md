# Generic Functions

- See syntax (28.1)
- Reduces code duplication
- Specifies what behaviour must be available

```rust
// Accepts a type with `Display` trait.
fn print<T: Display>(t: T) {
  println!("{}", t);
}

print(32);
print("Hello");
```
