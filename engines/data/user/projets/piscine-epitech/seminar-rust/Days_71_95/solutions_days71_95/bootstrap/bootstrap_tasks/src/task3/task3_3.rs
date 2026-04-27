pub fn parse_and_double(input: &str) -> Result<i32, String> {
    let number: i32 = input.trim().parse().map_err(|_| "Invalid number".to_string())?;
    Ok(number * 2)
}
