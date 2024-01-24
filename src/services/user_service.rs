pub struct UserService {}

impl UserService {
    pub fn new() -> UserService {
        UserService {}
    }

    pub fn do_something(&self, message: String) -> &Self {
        println!("Doing something with message: {}", message);
        self
    }
}
