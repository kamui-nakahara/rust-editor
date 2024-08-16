pub struct Settings {
    pub number: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self { number: true }
    }
}
