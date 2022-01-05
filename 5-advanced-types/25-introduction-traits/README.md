# Introduction to Traits

- Specifies behaviour without structure
- Similar to interfaces in Java
- Can force required methods and provide default methods
- Introduces the `self` keyword (see 25.1)


```rust
struct Foo {
  x: u32
}

trait Print {
  fn print(&self);
}

impl Print of Foo {

  fn print(&self) {
    println!("{}", self.x);
  }

}
```
