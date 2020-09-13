pub struct Player {
    name: &'static str
}

impl Player {
    pub fn new(name: &'static str) -> Self {
        Player {
            name
        }
    }
}
