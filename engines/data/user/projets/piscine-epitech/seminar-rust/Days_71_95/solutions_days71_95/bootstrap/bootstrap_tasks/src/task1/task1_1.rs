pub fn double(n: i32) -> i32 {
    n * 2
}

pub fn is_adult(age: u8) -> bool {
    age >= 18
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}