use cli::get_training_file;

pub enum Event {
    Join(Join),
    Message(Message),
}

pub struct Harris {
    //notes: HashMap<u64, HashMap<String, String>>

}

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

impl Harris {
    pub fn respond(&self, e: &Event) -> String {
        match e {
            &Event::Join(ref j) => {
                // Joining responds the same way every time. This could be extended to either a larger
                // list of interesting 'leads' or memory of a previous chat, etc.
                format!("Hello, {}! I can answer your weather questions.", j.name)
            },
            &Event::Message(ref m) => {
                m.text.to_owned()
            }
        }
    }
}

impl Default for Harris {
    fn default() -> Self {
        Self {

        }
    }
}