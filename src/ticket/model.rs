#[derive(Debug)]
pub enum State {
    ToDo,
    Blocked,
    Doing,
    Review,
    Done,
    Urgent,
}

#[derive(Debug)]
pub struct Ticket {
    pub id: usize,
    pub board_id: usize,
    pub title: String,
    pub description: String,
    pub state: State,
}
