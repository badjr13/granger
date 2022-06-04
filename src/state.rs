enum State {
    ToDo { limit: isize },
    Blocked { limit: isize },
    Doing { limit: i8 },
    Review { limit: i8 },
    Complete { limit: isize },
}

