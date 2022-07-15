#[allow(dead_code)]
#[derive(Debug)]
pub enum State {
    ToDo,
    Blocked,
    Doing,
    Review,
    Done,
}

impl State {
    pub fn get(&self) -> String {
        match self {
            Self::ToDo => String::from("todo"),
            Self::Blocked => String::from("blocked"),
            Self::Doing => String::from("doing"),
            Self::Review => String::from("review"),
            Self::Done => String::from("done"),
        }
    }
}

#[derive(Debug)]
pub struct Ticket {
    pub id: usize,
    pub board_id: usize,
    pub title: String,
    pub description: String,
    pub state: State,
}

impl Ticket {
    pub fn new(board_id: usize, title: String, description: String) -> Ticket {
        Ticket {
            id: 0,
            board_id,
            title,
            description,
            state: State::ToDo,
        }
    }
}
