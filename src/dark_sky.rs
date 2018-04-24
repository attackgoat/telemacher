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
        let response = get(&match dt {
            None => format!("https://api.darksky.net/forecast/{}/{},{}?exclude=alerts,flags", api_key, lat, lng),
            Some(dt) => format!("https://api.darksky.net/forecast/{}/{},{},{}?exclude=alerts,flags", api_key, lat, lng, dt.timestamp()),
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
        let mut daily = response["daily"]["summary"].as_str();
        let daily_humidity = response["daily"]["data"][0]["humidity"].as_number();
        let daily_uv_index = response["daily"]["data"][0]["uvIndex"].as_number();
        let daily_wind_speed = response["daily"]["data"][0]["windSpeed"].as_number();
        let daily_precip = response["daily"]["data"][0]["precipType"].as_str();

        // Hourly fields
        let hourly = response["hourly"]["summary"].as_str();
        let hourly_humidity = response["hourly"]["data"][0]["humidity"].as_number();
        let hourly_uv_index = response["hourly"]["data"][0]["uvIndex"].as_number();
        let hourly_wind_speed = response["hourly"]["data"][0]["windSpeed"].as_number();
        let hourly_precip = response["hourly"]["data"][0]["precipType"].as_str();

        // Minutely fields
        let mut minutely = response["minutely"]["summary"].as_str();
        let mut minutely_precip = response["minutely"]["data"][0]["precipType"].as_str();

        // Currently fields
        let currently = response["currently"]["summary"].as_str();
        let currently_humidity: Option<_> = response["currently"]["humidity"].as_number();
        let currently_uv_index: Option<_> = response["currently"]["uvIndex"].as_number();
        let currently_wind_speed: Option<_> = response["currently"]["windSpeed"].as_number();
        let currently_precip = response["currently"]["precipType"].as_str();

        // Fix up fields we may not have

        if let None = daily {
            daily = hourly;
        }

        if let None = minutely {
            minutely = currently;
        }

        if let None = minutely_precip {
            minutely_precip = currently_precip;
        }

        // Sanity check: fields should be somewhat reasonable - but most should be able to be blank by default
        if daily.is_none() || daily_humidity.is_none()
        || hourly.is_none() || hourly_humidity.is_none()
        || minutely.is_none()
        || currently.is_none() || currently_humidity.is_none() {
            return None;
        }

        // Default to none for precipitation because it can be missing
        let daily_precip = daily_precip.unwrap_or("none");
        let hourly_precip = hourly_precip.unwrap_or("none");
        let minutely_precip = minutely_precip.unwrap_or("none");
        let currently_precip = currently_precip.unwrap_or("none");

        Some(Forecast {
            daily: Prediction {
                humidity: daily_humidity.unwrap().into(),
                is_haily: daily_precip == "hail",
                is_rainy: daily_precip == "rain",
                is_snowy: daily_precip == "snow",
                uv_index: daily_uv_index.unwrap_or(0.into()).into(),
                summary: daily.unwrap().to_owned(),
                wind_speed: daily_wind_speed.unwrap_or(0.into()).into(),
            },
            hourly: Prediction {
                humidity: hourly_humidity.unwrap().into(),
                is_haily: hourly_precip == "hail",
                is_rainy: hourly_precip == "rain",
                is_snowy: hourly_precip == "snow",
                uv_index: hourly_uv_index.unwrap_or(0.into()).into(),
                summary: hourly.unwrap().to_owned(),
                wind_speed: hourly_wind_speed.unwrap_or(0.into()).into(),
            },
            minutely: Prediction {
                humidity: currently_humidity.unwrap().into(),
                is_haily: minutely_precip == "hail",
                is_rainy: minutely_precip == "rain",
                is_snowy: minutely_precip == "snow",
                uv_index: currently_uv_index.unwrap_or(0.into()).into(),
                summary: minutely.unwrap().to_owned(),
                wind_speed: currently_wind_speed.unwrap_or(0.into()).into(),
            },
            currently: Prediction {
                humidity: currently_humidity.unwrap().into(),
                is_haily: currently_precip == "hail",
                is_rainy: currently_precip == "rain",
                is_snowy: currently_precip == "snow",
                uv_index: currently_uv_index.unwrap_or(0.into()).into(),
                summary: currently.unwrap().to_owned(),
                wind_speed: currently_wind_speed.unwrap_or(0.into()).into(),
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