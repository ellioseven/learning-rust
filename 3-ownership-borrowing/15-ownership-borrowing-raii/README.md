# Ownership, Borrowing & RAII

- Ownership and borrowing prevent common pitfalls when working with memory and concurrency
- Ownership belongs to the initial binding, eg: `let x = new String()`
- Ownership can be passed to bindings functions, etc., but cannot use previous bindings (see 15.1)
- Ownership can be "borrowed" instead of moved (see 15.2)
- A value can be immutably borrowed any number of times
- A value can be mutably borrowed when only one binding exists and no other immutable binding exists (see 15.3)
  - Either one mutable or many immutable
- Resource Allocation Is Initilization
  - Resources (file, memory, etc.) are created on init. and freed on de-init (see 15.4)

```rust
// 15.1
let a = foo();

// Ownership passed from `a` to `b`.
let b = a;

// Ownership passed to `do_something()`.
do_something(b);

// ERROR: Ownership has already been passed to `do_something()` and can't be re-used.
do_something_else(b);
```

```rust
// 15.2
let a = foo();

// `b` borrows the value of `a`
// `a` still retains ownership of the value.
let b = &a;

// Function can also borrow `a`.
do_something(&a);
```

```rust
// 15.3
let a = foo()

// Create a mutable borrow.
let b = &mut a;

// ERROR: No other borrow can exist.
// let c = a;
// let c = &a;
```

```rust
// 15.4
fn wt () {
  // File resource created.
  let mut file = File::create("foo.txt");
  file.write_all("Hello, World!");
} // File closed automatically.
```
