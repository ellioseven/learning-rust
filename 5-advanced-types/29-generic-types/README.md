# Generic Types

- See syntax (29.1)
- Allow a type to be passed dynamically to another type (see 29.2)


```rust
// 29.1
struct Tagged<T> {
  value: T,
  tag: String
}

impl<T> Tagged<T> {
  fn tag(&self) -> String {
    self.tag.clone();
  }
}
```


```rust
// 29.2

// Vec methods can be used, across multiple types, across a single
// implementation.
let mut vi: Vec<i32> = vec![20];
let mut vs: Vec<&str> = vec!["Hi."];

vs.push("Hello);
vi.push(1024);
vs.pop();
```
