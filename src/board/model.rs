use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Board {
    id: u8,
    location: PathBuf,
    name: String,
}

impl Board {
    pub fn new(location: PathBuf, name: String) -> Board {
        Board {
            id: 0,
            location,
            name,
        }
    }
}
