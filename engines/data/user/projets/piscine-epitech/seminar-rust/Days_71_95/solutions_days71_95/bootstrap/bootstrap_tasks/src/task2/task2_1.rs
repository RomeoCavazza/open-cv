pub fn get_first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

pub fn make_uppercase(text: &str)-> String {
    text.to_ascii_uppercase()
}

pub fn count_chars(text: &str)-> usize {
    text.chars().count()    
}