const FIB_ZERO: u64 = 0;
const FIB_ONE: u64 = 1;

fn fib (n: u64) -> u64 {
    if n == FIB_ZERO {
        FIB_ZERO
    } else if n == FIB_ONE {
        FIB_ONE
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main () {
    for i in 0..21 {
        let num: u64 = fib(i);
        println!("{}: {}", i, num);
    }
}
