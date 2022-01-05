use std::collections::HashMap;

const FIB_ZERO: u64 = 0;
const FIB_ONE: u64 = 1;

// &mut is a mutable reference.
fn fib (index: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    // A match is a much for powerful switch statement.
    match index {
        FIB_ZERO => FIB_ZERO,
        FIB_ONE => FIB_ONE,
        index => {
            // `contains_key` expects a reference.
            if cache.contains_key(&index) {
                // Return contents of mutable reference at key.
                // Unwrap will consume the option and return a reference to `Some`, "panic" the
                // program if value not found. Use `*` to return conents instead of reference.
                // @url https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
                // @url https://doc.rust-lang.org/core/option/enum.Option.html#method.unwrap
                *cache.get(&index)
            } else {
                let value = fib(index - 1, cache) + fib(index - 2, cache);
                cache.insert(index, value);
                value // Evauluate and return value from function.
            }
        }
    }
}

fn main () {
    // Create new mutable hash map.
    let mut cache = HashMap::new();
    for index in 0..21 {
        // Pass index and mutable reference to mutable hash map.
        let value = fib(index, &mut cache);
        println!("{}: {}", index, value);
    }
}
