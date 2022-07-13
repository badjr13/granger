#[derive(Debug)]
pub struct Board {
    pub id: usize,
    pub name: String,
    pub location: String,
}

impl Board {
    pub fn new(name: &String, location: &String) -> Board {
        Board {
            id: 0,
            name: name.to_string(),
            location: location.to_string(),
        }
    }
}
