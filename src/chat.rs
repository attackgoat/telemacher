enum Request {
    Join(Join),
    Message(Message),
}

struct Join {
    name: String,
    user_id: u64,
}

struct Message {
    text: String,
    user_id: u64,
}

struct Response {
    messages: Vec<String>,
}