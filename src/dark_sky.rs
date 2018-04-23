use chrono::{DateTime, TimeZone};

use reqwest::get;

use cli::get_dark_sky_api_key;

pub struct DarkSkyApi {
    //cache: RefCell<LruCache<.., ..>>,
    // Caching (locally or via some redis like instance
    // would be a big win here - but we'd need to keep
    // in mind how long each forecast might be trusted
    // for and other issues which make this feature fun
    // but extra). See GoogleApi type for details.
}

impl DarkSkyApi {
    pub fn try_get_forecast<T: TimeZone>(&self, lat: f64, lng: f64, dt: Option<DateTime<T>>) -> () {
        // Make a web request to Dark Sky asking for this data
        let api_key = get_dark_sky_api_key();
        let mut response = get(&match dt {
            None => format!("https://api.darksky.net/forecast/{}/{},{}&exclude=alerts,flags", api_key, lat, lng),
            Some(dt) => format!("https://api.darksky.net/forecast/{}/{},{},{}&exclude=alerts,flags", api_key, lat, lng, dt.timestamp()),
        });
        if response.is_err() {
            return ();
        }

        ()
    }
}

impl Default for DarkSkyApi {
    fn default() -> Self {
        Self {
        }
    }
}