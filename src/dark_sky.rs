use chrono::{DateTime, TimeZone};

use json;

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
    pub fn try_get_forecast<T: TimeZone>(&self, lat: f64, lng: f64, dt: Option<DateTime<T>>) -> Option<Forecast> {
        // Make a web request to Dark Sky asking for this data
        let api_key = get_dark_sky_api_key();
        let mut response = get(&match dt {
            None => format!("https://api.darksky.net/forecast/{}/{},{}&exclude=alerts,flags", api_key, lat, lng),
            Some(dt) => format!("https://api.darksky.net/forecast/{}/{},{},{}&exclude=alerts,flags", api_key, lat, lng, dt.timestamp()),
        });
        
        // Sanity check: We have some text at least
        let response = response.unwrap().text();
        if response.is_err() {
            return None;
        }

        // Parse the output json structure
        let response = json::parse(&response.unwrap());

        // Sanity check: Good json
        if response.is_err() {
            return None;
        }

        let response = response.unwrap();

        // Daily fields
        let daily = &response["daily"]["summary"].as_str();
        let daily_humidity = &response["daily"]["humidity"].as_f32();
        let daily_uv_index = &response["daily"]["uvIndex"].as_u8();
        let daily_wind_speed = &response["daily"]["windSpeed"].as_f32();
        let daily_precip = &response["daily"]["precipType"].as_str();

        // Hourly fields
        let hourly = &response["hourly"]["summary"].as_str();
        let hourly_humidity = &response["hourly"]["humidity"].as_f32();
        let hourly_uv_index = &response["hourly"]["uvIndex"].as_u8();
        let hourly_wind_speed = &response["hourly"]["windSpeed"].as_f32();
        let hourly_precip = &response["hourly"]["precipType"].as_str();

        // Minutely fields
        let minutely = &response["minutely"]["summary"].as_str();
        let minutely_humidity = &response["currently"]["humidity"].as_f32();    // This field doesn't exist for minutely
        let minutely_uv_index = &response["currently"]["uvIndex"].as_u8();      // This field doesn't exist for minutely
        let minutely_wind_speed = &response["currently"]["windSpeed"].as_f32(); // This field doesn't exist for minutely
        let minutely_precip = &response["minutely"]["precipType"].as_str();

        // Currently fields
        let currently = &response["currently"]["summary"].as_str();
        let currently_humidity = &response["currently"]["humidity"].as_f32();
        let currently_uv_index = &response["currently"]["uvIndex"].as_u8();
        let currently_wind_speed = &response["currently"]["windSpeed"].as_f32();
        let currently_precip = &response["currently"]["precipType"].as_str();

        // Sanity check: fields should be somewhat reasonable
        if daily.is_none() || daily_humidity.is_none() || daily_precip.is_none() || daily_uv_index.is_none() || daily_wind_speed.is_none()
        || hourly.is_none() || hourly_humidity.is_none() || hourly_precip.is_none() || hourly_uv_index.is_none() || hourly_wind_speed.is_none()
        || minutely.is_none() || minutely_humidity.is_none() || minutely_precip.is_none() || minutely_uv_index.is_none() || minutely_wind_speed.is_none()
        || currently.is_none() || currently_humidity.is_none() || currently_precip.is_none() || currently_uv_index.is_none() || currently_wind_speed.is_none() {
            return None;
        }

        let daily_precip = daily_precip.unwrap();
        let hourly_precip = hourly_precip.unwrap();
        let minutely_precip = minutely_precip.unwrap();
        let currently_precip = currently_precip.unwrap();

        Some(Forecast {
            daily: Prediction {
                humidity: daily_humidity.unwrap(),
                is_haily: daily_precip == "hail",
                is_rainy: daily_precip == "rain",
                is_snowy: daily_precip == "snow",
                uv_index: daily_uv_index.unwrap(),
                summary: daily.unwrap().to_owned(),
                wind_speed: daily_wind_speed.unwrap(),
            },
            hourly: Prediction {
                humidity: hourly_humidity.unwrap(),
                is_haily: hourly_precip == "hail",
                is_rainy: hourly_precip == "rain",
                is_snowy: hourly_precip == "snow",
                uv_index: hourly_uv_index.unwrap(),
                summary: hourly.unwrap().to_owned(),
                wind_speed: hourly_wind_speed.unwrap(),
            },
            minutely: Prediction {
                humidity: minutely_humidity.unwrap(),
                is_haily: minutely_precip == "hail",
                is_rainy: minutely_precip == "rain",
                is_snowy: minutely_precip == "snow",
                uv_index: minutely_uv_index.unwrap(),
                summary: minutely.unwrap().to_owned(),
                wind_speed: minutely_wind_speed.unwrap(),
            },
            currently: Prediction {
                humidity: currently_humidity.unwrap(),
                is_haily: currently_precip == "hail",
                is_rainy: currently_precip == "rain",
                is_snowy: currently_precip == "snow",
                uv_index: currently_uv_index.unwrap(),
                summary: currently.unwrap().to_owned(),
                wind_speed: currently_wind_speed.unwrap(),
            }
        })
    }
}

impl Default for DarkSkyApi {
    fn default() -> Self {
        Self {
        }
    }
}

pub struct Forecast {
    pub daily: Prediction,
    pub hourly: Prediction,
    pub minutely: Prediction,
    pub currently: Prediction,
}

pub struct Prediction {
    pub humidity: f32,
    pub is_haily: bool,
    pub is_rainy: bool,
    pub is_snowy: bool,
    pub uv_index: u8,
    pub summary: String,
    pub wind_speed: f32,
}