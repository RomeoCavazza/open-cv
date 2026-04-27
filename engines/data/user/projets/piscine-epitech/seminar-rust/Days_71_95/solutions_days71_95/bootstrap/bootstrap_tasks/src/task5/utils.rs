pub fn is_valid_username(username:&str) -> bool {
    username.len() >= 3 && username.len() <= 20
}