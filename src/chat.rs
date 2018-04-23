pub struct Join {
    name: String,
    user_id: u64,
}

impl Join {
    pub fn new<N: Into<String>>(user_id: u64, name: N) -> Self {
        Self {
            name: name.into(),
            user_id: user_id,
        }
    }
}

pub struct Message {
    text: String,
    user_id: u64,
}

impl Message {
    pub fn new<N: Into<String>>(user_id: u64, text: N) -> Self {
        Self {
            text: text.into(),
            user_id: user_id,
        }
    }
}

pub enum Request {
    Join(Join),
    Message(Message),
}

pub struct Server {

}

impl Server {
    pub fn respond(&self, request: &Request) -> String {
        String::new()
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {

        }
    }
}