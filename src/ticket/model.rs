use serde::Serialize;

const TODO: &str = "todo";
const BLOCKED: &str = "blocked";
const DOING: &str = "doing";
const REVIEW: &str = "review";
const DONE: &str = "done";

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum State {
    ToDo,
    Blocked,
    Doing,
    Review,
    Done,
}

impl State {
    pub fn value(&self) -> String {
        match self {
            Self::ToDo => String::from(TODO),
            Self::Blocked => String::from(BLOCKED),
            Self::Doing => String::from(DOING),
            Self::Review => String::from(REVIEW),
            Self::Done => String::from(DONE),
        }
    }

    pub fn from_string(state_as_string: &str) -> State {
        match state_as_string {
            TODO => State::ToDo,
            BLOCKED => State::Blocked,
            DOING => State::Doing,
            REVIEW => State::Review,
            DONE => State::Done,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Ticket {
    #[serde(skip_serializing)]
    pub id: usize,

    #[serde(skip_serializing)]
    pub board_id: usize,

    pub title: String,

    pub description: String,

    #[serde(skip_serializing)]
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
