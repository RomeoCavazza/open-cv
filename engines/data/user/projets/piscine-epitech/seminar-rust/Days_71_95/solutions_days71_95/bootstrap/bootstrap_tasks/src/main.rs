use std::collections::HashMap;

mod task1;
mod task2;
mod task3;
mod task4;
mod task5;

fn main() {
    // Task 1.1
    println!("{}", task1::task1_1::double(5));
    println!("{}", task1::task1_1::is_adult(20));
    println!("{}", task1::task1_1::greet("Romeo"));

    // Task 1.2
    println!("{:?}", task1::task1_2::increment_all(vec![1, 2, 3]));

    // Task 2.1
    println!("{}", task2::task2_1::get_first_word("Hello Rust world"));
    println!("{}", task2::task2_1::make_uppercase("Hello Rust world"));
    println!("{}", task2::task2_1::count_chars("Hello Rust world"));

    // Task 2.2
    let mut s = String::from("Hello");
    task2::task2_2::append_suffix(&mut s, " world!");
    println!("{}", s);

    // Task 2.3
    let words = vec!["rust".to_string(), "rocket".to_string(), "python".to_string()];
    println!("{:?}", task2::task2_3::find_by_prefix(&words, "ro"));
    println!("{}", task2::task2_3::total_length(&words));

    // Task 3.1
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", task3::task3_1::find_max(numbers));
    
    let strings = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
    println!("{:?}", task3::task3_1::get_at_index(strings, 1));

    // Task 3.2
    println!("{:?}", task3::task3_2::divide(10.0, 2.0));
    println!("{:?}", task3::task3_2::divide(10.0, 0.0));
    println!("{:?}", task3::task3_2::parse_age("25"));
    println!("{:?}", task3::task3_2::parse_age("150"));
    println!("{:?}", task3::task3_2::parse_age("abc"));

    // Task 3.3
    println!("{:?}", task3::task3_3::parse_and_double("21"));
    println!("{:?}", task3::task3_3::parse_and_double("abc"));

    // Task 4.1
    let person = task4::task4_1::Person::new("Alice".to_string(), 25, "alice@example.com".to_string());
    println!("{}", person.is_adult());
    println!("{}", person.display());

    // Task 4.2
    let mut contacts: HashMap<String, String> = HashMap::new();
    task4::task4_2::add_contact(&mut contacts, "Alice".to_string(), "0612345678".to_string());
    task4::task4_2::add_contact(&mut contacts, "Bob".to_string(), "0698765432".to_string());
    println!("{:?}", contacts);
    println!("{}", task4::task4_2::remove_contact(&mut contacts, "Alice"));
    println!("{}", task4::task4_2::remove_contact(&mut contacts, "Charlie"));
    println!("{:?}", contacts);

    // Task 5.1
    let user = task5::models::User {
        id: 1,
        username: "rustacean".to_string(),
    };
    println!("{:?}", user);
    println!("User ID: {}", user.id);
    println!("{}", task5::utils::is_valid_username(&user.username));
    println!("{}", task5::utils::is_valid_username("ab"));
}