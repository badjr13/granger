use std::path::PathBuf;

#[derive(Debug)]
pub struct Board {
    pub id: u8,
    pub location: PathBuf,
    pub name: String,
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
