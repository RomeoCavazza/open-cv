use std::collections::HashMap;

pub fn add_contact(map: &mut HashMap<String, String>, name: String, phone: String) {
    map.insert(name, phone);
}

pub fn remove_contact(map: &mut HashMap<String, String>, name: &str) -> bool {
    map.remove(name).is_some()
}
