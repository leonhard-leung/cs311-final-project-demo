// #[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

impl User {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}
// pub fn main() {
//     let user1 = User {
//         name: String::from("Alice"),
//         age: 30,
//     };
//
//     // Debug trait allows us to print the struct
//     println!("{:?}", user1);
//
//     // Clone trait allows us to duplicate the struct
//     let mut user2 = user1.clone();
//     // user2.set_age(300);
//     println!("{:?}", user2);
//
//     println!("Are users equal? {}", user1 ==user2);
// }
