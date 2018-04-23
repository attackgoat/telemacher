use chrono::Local;

use snips_nlu_lib::{FileBasedConfiguration, SnipsNluEngine};
use snips_nlu_ontology::{Slot, SlotValue};

use cli::get_training_file;
use dark_sky::try_get_forecast;
use google_maps::try_get_lat_lng;

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
                let nlu = self.nlu_engine.parse(&m.text, None).expect("nlu failure");
                match &nlu.intent {
                    &Some(ref i) if i.probability > 0.5
                                    && nlu.slots.is_some()
                                    && (&i.intent_name == "searchWeatherForecast" || &i.intent_name == "searchWeatherForecastCondition") => {
                                        Self::respond_weather(&nlu.slots.unwrap())
                                    },
                    _ => Self::respond_unsure(),
                }
            }
        }
    }

    fn respond_down() -> String {
        "Something went terribly wrong deep inside my logic. Put me on the floor and step back.".to_owned()
    }

    fn respond_unsure() -> String {
        // TODO: Add some variety or memory of past utterances
        "Hmm. That is fascinating. Ask me about the weather where you live.".to_owned()
    }

    fn respond_weather(slots: &[Slot]) -> String {
        // Pick out the values from slots; could be simpler but this form allows for the use of all
        // value formats offered by the library and a way to handle each type according to our needs
        let mut forecast_condition_name = None;
        let mut forecast_country = None;
        let mut forecast_geographical_poi = None;
        let mut forecast_locality = None;
        let mut forecast_region = None;
        let mut forecast_start_datetime = None;
        for slot in slots {
            match slot {
                s if &s.slot_name == "forecast_condition_name" => {
                    if let &SlotValue::Custom(ref v) = &s.value {
                        forecast_condition_name = Some(v.value.to_owned());
                    }
                },
                s if &s.slot_name == "forecast_country" => {
                    if let &SlotValue::Custom(ref v) = &s.value {
                        forecast_country = Some(v.value.to_owned());
                    }
                },
                s if &s.slot_name == "forecast_geographical_poi" => {
                    if let &SlotValue::Custom(ref v) = &s.value {
                        forecast_geographical_poi = Some(v.value.to_owned());
                    }
                },
                s if &s.slot_name == "forecast_locality" => {
                    if let &SlotValue::Custom(ref v) = &s.value {
                        forecast_locality = Some(v.value.to_owned());
                    }
                },
                s if &s.slot_name == "forecast_region" => {
                    if let &SlotValue::Custom(ref v) = &s.value {
                        forecast_region = Some(v.value.to_owned());
                    }
                },
                s if &s.slot_name == "forecast_start_datetime" => {
                    if let &SlotValue::InstantTime(ref v) = &s.value {
                        forecast_start_datetime = Some(v.value.to_owned());
                    }
                },
                _ => (),
            }
        }

        // We have four location values so we preference locality, poi, region, then country
        if let None = forecast_locality {
            forecast_locality = if let Some(v) = forecast_geographical_poi {
                Some(v)
            } else if let Some(v) = forecast_region {
                Some(v)
            } else if let Some(v) = forecast_country {
                Some(v)
            } else {
                None
            };
        }

        // Sanity check: We should have found a location
        // A real service might guess using a commercial product such as https://www.maxmind.com
        if let None = forecast_locality {
            return Self::respond_unsure();
        }

        // If we didn't receive a date/time for the weather forecast then we should use now
        if let None = forecast_start_datetime {
            forecast_start_datetime = Some(Local::now().to_string());
        }

        let forecast_locality = forecast_locality.unwrap();
        let forecast_start_datetime = forecast_start_datetime.unwrap();

        // See if we can further answer their specific question (these items must be in the training set)
        enum Forecast {
            Snow,
            Wind,
            Hail,
            Humidity,
            Precipitation,
            Uv,
        };
        let desired_forecast = match &forecast_condition_name {
            &Some(ref v) if v == "blizzard"
                         || v == "snow"
                         || v == "snowfall"
                         || v == "snowing"
                         || v == "snowstorm"
                         || v == "snowy" => Some(Forecast::Snow),
            &Some(ref v) if v == "wind"
                         || v == "windy" => Some(Forecast::Wind),
            &Some(ref v) if v == "hail"
                         || v == "hailing" => Some(Forecast::Hail),
            &Some(ref v) if v == "humid" => Some(Forecast::Humidity),
            &Some(ref v) if v == "storm"
                         || v == "stormy"
                         || v == "rain"
                         || v == "rainfall"
                         || v == "rainy" => Some(Forecast::Precipitation),
            &Some(ref v) if v == "cloud"
                         || v == "cloudi"
                         || v == "overcast"
                         || v == "depress"
                         || v == "fog"
                         || v == "foggy"
                         || v == "sun"
                         || v == "sunni"
                         || v == "hot"
                         || v == "be sunni" => Some(Forecast::Uv),
            _ => None,
        };

        // At this point we know they're asking about weather. We also have:
        // forecast_locality: String
        // forecast_start_datetime: String
        // desired_forecast: Option<Forecast>

        // Step 1: Process locality string into lat/lng
        let lat_lng = try_get_lat_lng(&forecast_locality);

        // Sanity check: We may have been unable to do that
        if let None = lat_lng {
            return Self::respond_down();
        }

        // Step 2: Go check the weather
        let (lat, lng) = lat_lng.unwrap();
        let forecast = try_get_forecast(lat, lng);

        "".to_owned()
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