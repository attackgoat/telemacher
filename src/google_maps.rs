use json::JsonValue;

use reqwest::get;

use urlencoding::encode;

use cli::get_google_api_key;

pub fn try_get_lat_lng(query: &str) -> Option<(f64, f64)> {
    // Make a web request to Google asking for this data
    let api_key = get_google_api_key();
    let mut response = get(&format!("https://maps.googleapis.com/maps/api/place/textsearch/json?query={}&key={}", encode(query), api_key));
    if response.is_err() {
        return None;
    }

    // Sanity check: We have some text at least
    let response = response.unwrap().text();
    if response.is_err() {
        return None;
    }

    // Parse the output json structure
    let response: JsonValue = response.unwrap().into();
    let location = &response["results"][0]["geometry"]["location"];
    let lat = &location["lat"];
    let lng = &location["lng"];

    // Sanity check: lat & lng must look like numbers at least
    if lat.is_null() || lng.is_null() || !lat.is_number() || !lng.is_number() {
        return None;
    }

    let lat = lat.as_f64().unwrap();
    let lng = lng.as_f64().unwrap();

    Some((lat, lng))
}