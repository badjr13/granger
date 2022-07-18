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

    pub fn from_sql(state_as_string: &str) -> State {
        match state_as_string {
            "todo" => State::ToDo,
            "blocked" => State::Blocked,
            "doing" => State::Doing,
            "review" => State::Review,
            "done" => State::Done,
            _ => todo!(),
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
