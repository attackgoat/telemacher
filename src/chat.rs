use cli::get_training_file;

use snips_nlu_lib::{FileBasedConfiguration, SnipsNluEngine};

pub enum Event {
    Join(Join),
    Message(Message),
}

pub struct Harris {
    //notes: HashMap<u64, HashMap<String, String>>
    nlu_engine: SnipsNluEngine,
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
                // https://goo.gl/rFY8XX
                format!("Hello, {}, this is Harris. I'm in right now, so you can talk to me personally.", j.name)
            },
            &Event::Message(ref m) => {
                let _ = self.nlu_engine.parse(&m.text, None).expect("nlu failure");

                //println!("{}", serde_json::to_string_pretty(&result).unwrap());
                m.text.to_owned()
            }
        }
    }
}

impl Default for Harris {
    fn default() -> Self {
        let config = FileBasedConfiguration::from_path(get_training_file(), false).expect("Unacceptable training file");
        let nlu_engine = SnipsNluEngine::new(config).expect("Unacceptable nlu configuration");

        Self {
            nlu_engine: nlu_engine,
        }
    }
}