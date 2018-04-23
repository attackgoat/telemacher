pub struct Join {
    name: String,
    #[allow(dead_code)] user_id: u64,
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
    #[allow(dead_code)] user_id: u64,
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
    //notes: HashMap<u64, HashMap<String, String>>
}

impl Server {
    pub fn respond(&self, request: &Request) -> String {
        match request {
            &Request::Join(ref j) => {
                format!("Hello, {}! I can answer your weather questions.", j.name)
            },
            &Request::Message(ref m) => {
                m.text.to_owned()
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {

        }
    }
}