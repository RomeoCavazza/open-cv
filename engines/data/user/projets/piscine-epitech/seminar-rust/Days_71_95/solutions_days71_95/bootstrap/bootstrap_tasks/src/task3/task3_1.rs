pub fn find_max(numbers: Vec<i32>) -> Option<i32> {
    numbers.iter().max().copied()
}

pub fn get_at_index(vec: Vec<String>, index: usize) -> Option<String> {
    vec.get(index).cloned()
}