fn main() {
    let user1 = User {
        username: String::from("name1"),
        email: String::from("1@email.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    let mut user2 = User {
        username: String::from("name2"),
        email: String::from("2@email.com"),
        ..user1
    };

    println!("{}", user2.username);
    user2.username = String::from("user2_new");
    println!("{}", user2.username);

    let mut user3 = User::new(String::from("name3"), String::from("3@email.com"));
    println!("{}", user3.username);
    user3.username = "name3_new".to_string();
    println!("{}", user3.username);

    user3.flip_active_state();
    println!("{}", user3.active);

    user3.set_sign_in_count(4);
    println!("{}", user3.sign_in_count);
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

impl User {
    // no need to return mutable User
    // mutability is determined by variable it is assigned to!!
    fn new(username: String, email: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn flip_active_state(&mut self) {
        self.active = !self.active;
    }
}

impl User {
    fn set_sign_in_count(&mut self, new_count: u32) {
        self.sign_in_count = new_count;
    }
}
