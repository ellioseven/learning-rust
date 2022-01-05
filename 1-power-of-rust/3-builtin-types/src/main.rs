fn main () {
    let a = 2;
    let b = 3;
    println!("{} + {} = {}", a, b, a + b);

    // Example of inferred types.
    let x = false;
    let y = x && true;
    println!("y is {}", y);

    // Example array usage.
    let arr: [i8; 3] =  [1, 4, 7];
    println!("{}", arr[0]);

    // Example tuple usage.
    let tpl: (i32, char) = (21, 'x');
    println!("{}", tpl.1);

    println!("2 + 2 = {}", add(2, 2));
}

fn add (a: i32, b: i64) -> i32 {
    // `as` is an example of type casting.
    return a + (b as i32);
}
