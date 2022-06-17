use std::path::PathBuf;

#[derive(Debug)]
pub struct Board {
    id: u8,
    location: PathBuf,
}

impl Board {
    pub fn new(location: PathBuf) -> Self {
        Self { id: 0, location }
    }
}
