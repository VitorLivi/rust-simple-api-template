pub struct EntityService {}

impl EntityService {
    pub fn new() -> EntityService {
        EntityService {}
    }

    pub fn do_something(&self, message: String) -> &Self {
        println!("Doing something with message: {}", message);
        self
    }
}
