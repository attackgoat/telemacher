use chrono::{DateTime, FixedOffset, Local};

use snips_nlu_lib::{FileBasedConfiguration, SnipsNluEngine};
use snips_nlu_ontology::{Slot, SlotValue, Grain};

use cli::get_training_file;
use dark_sky::{DarkSkyApi, Forecast, Prediction};
use google::GoogleApi;

pub enum Event {
    Join(Join),
    Message(Message),
}

pub struct Harris {
    dark_sky_api: DarkSkyApi,
    google_api: GoogleApi,
    nlu_engine: SnipsNluEngine,
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
                                        self.respond_weather(&nlu.slots.unwrap())
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

    fn respond_weather(&self, slots: &[Slot]) -> String {
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
                        if let Ok(d) = DateTime::<FixedOffset>::parse_from_str(&v.value, "%Y-%m-%d %H:%M:%S %:z") {
                            forecast_start_datetime = Some((d, v.grain));
                        }
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

        let forecast_locality = forecast_locality.unwrap();

        // See if we can further answer their specific question (these items must be in the training set)
        enum SpecificForecast {
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
                         || v == "snowy" => Some(SpecificForecast::Snow),
            &Some(ref v) if v == "wind"
                         || v == "windy" => Some(SpecificForecast::Wind),
            &Some(ref v) if v == "hail"
                         || v == "hailing" => Some(SpecificForecast::Hail),
            &Some(ref v) if v == "humid" => Some(SpecificForecast::Humidity),
            &Some(ref v) if v == "storm"
                         || v == "stormy"
                         || v == "rain"
                         || v == "rainfall"
                         || v == "rainy" => Some(SpecificForecast::Precipitation),
            &Some(ref v) if v == "cloud"
                         || v == "cloudi"
                         || v == "overcast"
                         || v == "depress"
                         || v == "fog"
                         || v == "foggy"
                         || v == "sun"
                         || v == "sunni"
                         || v == "hot"
                         || v == "be sunni" => Some(SpecificForecast::Uv),
            _ => None,
        };

        // At this point we know they're asking about weather. We also have:
        // forecast_locality: String
        // forecast_start_datetime: Option<(DateTime<FixedOffset>, Grain)>
        // desired_forecast: Option<DesiredForecast>

        // Step 1: Process locality string into lat/lng
        let lat_lng = self.google_api.try_get_lat_lng(&forecast_locality);

        // Sanity check: We may have been unable to do that
        if let None = lat_lng {
            return Self::respond_down();
        }

        // Step 2: Go check the weather
        let (lat, lng) = lat_lng.unwrap();
        let mut dt = None;
        let mut grain = None;
        if let Some((d, g)) = forecast_start_datetime {
            dt = Some(d.clone());
            grain = Some(g);
        }
        let forecast = self.dark_sky_api.try_get_forecast(lat, lng, dt);

        // Step 3: Pick the correct combination of desired forecast and granularity so we can respond
        match desired_forecast {
            None => {
                match grain {
                    None | Some(Grain::Second) => format!("currently").to_owned(),
                    Some(Grain::Minute) => format!("minutely").to_owned(),
                    Some(Grain::Hour) => format!("hourly").to_owned(),
                    Some(Grain::Year) | Some(Grain::Quarter) | Some(Grain::Month) | Some(Grain::Week) | Some(Grain::Day) => format!("daily").to_owned(),
                }
            },
            Some(SpecificForecast::Hail) => {
                match grain {
                    None | Some(Grain::Second) => format!("currently").to_owned(),
                    Some(Grain::Minute) => format!("minutely").to_owned(),
                    Some(Grain::Hour) => format!("hourly").to_owned(),
                    Some(Grain::Year) | Some(Grain::Quarter) | Some(Grain::Month) | Some(Grain::Week) | Some(Grain::Day) => format!("daily").to_owned(),
                }
            },
            Some(SpecificForecast::Humidity) => {
                match grain {
                    None | Some(Grain::Second) => format!("currently").to_owned(),
                    Some(Grain::Minute) => format!("minutely").to_owned(),
                    Some(Grain::Hour) => format!("hourly").to_owned(),
                    Some(Grain::Year) | Some(Grain::Quarter) | Some(Grain::Month) | Some(Grain::Week) | Some(Grain::Day) => format!("daily").to_owned(),
                }
            },
            Some(SpecificForecast::Precipitation) => {
                match grain {
                    None | Some(Grain::Second) => format!("currently").to_owned(),
                    Some(Grain::Minute) => format!("minutely").to_owned(),
                    Some(Grain::Hour) => format!("hourly").to_owned(),
                    Some(Grain::Year) | Some(Grain::Quarter) | Some(Grain::Month) | Some(Grain::Week) | Some(Grain::Day) => format!("daily").to_owned(),
                }
            },
            Some(SpecificForecast::Snow) => {
                match grain {
                    None | Some(Grain::Second) => format!("currently").to_owned(),
                    Some(Grain::Minute) => format!("minutely").to_owned(),
                    Some(Grain::Hour) => format!("hourly").to_owned(),
                    Some(Grain::Year) | Some(Grain::Quarter) | Some(Grain::Month) | Some(Grain::Week) | Some(Grain::Day) => format!("daily").to_owned(),
                }
            },
            Some(SpecificForecast::Uv) => {
                match grain {
                    None | Some(Grain::Second) => format!("currently").to_owned(),
                    Some(Grain::Minute) => format!("minutely").to_owned(),
                    Some(Grain::Hour) => format!("hourly").to_owned(),
                    Some(Grain::Year) | Some(Grain::Quarter) | Some(Grain::Month) | Some(Grain::Week) | Some(Grain::Day) => format!("daily").to_owned(),
                }
            },
            Some(SpecificForecast::Wind) => {
                match grain {
                    None | Some(Grain::Second) => format!("currently").to_owned(),
                    Some(Grain::Minute) => format!("minutely").to_owned(),
                    Some(Grain::Hour) => format!("hourly").to_owned(),
                    Some(Grain::Year) | Some(Grain::Quarter) | Some(Grain::Month) | Some(Grain::Week) | Some(Grain::Day) => format!("daily").to_owned(),
                }
            },
        }
    }
}

impl Default for Harris {
    fn default() -> Self {
        let config = FileBasedConfiguration::from_path(get_training_file(), false).expect("Unacceptable training file");
        let nlu_engine = SnipsNluEngine::new(config).expect("Unacceptable nlu configuration");

        Self {
            dark_sky_api: Default::default(),
            google_api: Default::default(),
            nlu_engine: nlu_engine,
        }
    }
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