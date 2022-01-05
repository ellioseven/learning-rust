fn main () {
    // Create a slice.
    let series = vec![1000, 5, 25];
    // Vector slice (borrowed reference) on index 1 to 2.
    // -> &[5, 25]
    let sla = &series[1..2];

    let name = String::from("Elliot");
    // String slice (borrowed reference) on index 1 to 2.
    // -> "ll"
    let slb = &name[1..2];
}
