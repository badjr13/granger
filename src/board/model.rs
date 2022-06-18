use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Board {
    id: u8,
    location: PathBuf,
    name: String,
}

impl Board {
    pub fn new(location: PathBuf) -> Board {
        let name = get_git_repository_name(&location);
        Board {
            id: 0,
            location,
            name,
        }
    }
}

fn get_git_repository_name(location: &Path) -> String {
    let mut output: Vec<&str> = location.to_str().unwrap().split('/').collect();
    output.pop().unwrap().to_string()
}
