fn main () {
    // Create mutatable vector in heap memory.
    let mut series = vec![1, 2, 1000, 4, 5];
    
    // ERROR: Can't have both mutable and immutable references.
    // let c = &series;
    
    // Pass mutable borrow to `series_cap_m()`.
    series_cap_m(10, &mut series);

    for value in series { println!("{}", value); }
}

fn series_cap_m (max: i32, series: &mut Vec<i32>) {
    for index in 0..series.len() {
        if series[index] > max {
            series[index] = max;
        }
    }
} // Return borrow reference.
