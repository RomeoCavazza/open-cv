pub struct Person {
    name: String,
    age: u8,
    email: String,
}

impl Person {
    pub fn new(name: String, age: u8, email: String) -> Self {
        Self { name, age, email }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    pub fn display(&self) -> String {
        format!("{} ({}) - {}", self.name, self.age, self.email)
    }
}