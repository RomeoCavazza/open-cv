pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn parse_age(input: &str) -> Result<u8, String> {
    match input.trim().parse::<u8>() {
        Ok(age) if age <= 120 => Ok(age),
        Ok(_) => Err("Age must be between 0 and 120".to_string()),
        Err(_) => Err("Invalid number format".to_string()),
    }
}
