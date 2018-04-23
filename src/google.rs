use std::cell::RefCell;
    
use json;

use lru_cache::LruCache;

use reqwest::get;

use urlencoding::encode;

use cli::get_google_api_key;

const LRU_CACHE_SIZE: usize = 16_384;

pub struct GoogleApi {
    cache: RefCell<LruCache<String, (f64, f64)>>,
}

impl GoogleApi {
    pub fn try_get_lat_lng(&self, query: &str) -> Option<(f64, f64)> {
        // See if we can short-cut all of this using our cache
        // This would be better if it were an external redis..
        {
            let mut cache = self.cache.borrow_mut();
            if let Some(&mut (lat, lng)) = cache.get_mut(query) {
                return Some((lat, lng));
            }
        }

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
        let response = json::parse(&response.unwrap());

        // Sanity check: Good json
        if response.is_err() {
            return None;
        }

        // Grab the fields we want
        let response = response.unwrap();
        let location = &response["results"][0]["geometry"]["location"];
        let lat = &location["lat"];
        let lng = &location["lng"];

        // Sanity check: lat & lng must look like numbers at least
        if lat.is_null() || lng.is_null() || !lat.is_number() || !lng.is_number() {
            return None;
        }

        let lat = lat.as_f64().unwrap();
        let lng = lng.as_f64().unwrap();

        // Stick the results back into our 'redis'
        {
            let mut cache = self.cache.borrow_mut();
            cache.insert(query.to_owned(), (lat, lng));
        }

        Some((lat, lng))
    }
}

impl Default for GoogleApi {
    fn default() -> Self {
        Self {
            cache: RefCell::new(LruCache::new(LRU_CACHE_SIZE))
        }
    }
}