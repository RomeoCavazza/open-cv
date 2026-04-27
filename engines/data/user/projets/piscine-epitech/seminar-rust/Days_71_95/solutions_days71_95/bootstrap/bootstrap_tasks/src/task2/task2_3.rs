pub fn find_by_prefix<'a>(words: &'a Vec<String>, prefix: &'a str) -> Option<&'a String> {
words.iter().find(|w| w.starts_with(prefix))
}

pub fn total_length(words: &Vec<String>) -> usize {
    words.iter().map(|w| w.len()).sum()
}