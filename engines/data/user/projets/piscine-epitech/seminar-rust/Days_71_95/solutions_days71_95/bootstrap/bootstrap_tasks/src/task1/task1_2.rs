pub fn increment_all(mut numbers: Vec<i32>) -> Vec<i32> {
    for n in &mut numbers {
        *n += 1;
    }
    numbers
}
