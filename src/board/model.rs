#[derive(Debug)]
pub struct Board {
    pub id: u8,
    pub name: String,
    pub location: String,
}

impl Board {
    pub fn new(name: String, location: String) -> Board {
        Board {
            id: 0,
            name,
            location,
        }
    }
}
