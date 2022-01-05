fn main () {
    // Initialise vector binding.
    let series = vec![1, 2, 3];
    // Borrow immutable reference to `sum()`.
    // Borrow prevents a move error.
    let result = sum(&series);
    println!("Sum: {}", result);
}

fn sum (series: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in series { sum += value; }
    return sum;
}
