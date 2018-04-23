use clap::App;

fn app_value_of(key: &str) -> Option<String> {
    // Load the command-line-argument-parser (CLAP) library
    let cli = load_yaml!("../cli.yml");
    let app = App::from_yaml(&cli).get_matches();

    // The load_yaml! macro runs at compile time so these loads are
    // just in-memory structure creations - very fast/no disk.

    if let Some(v) = app.value_of(key) {
        Some(v.to_owned())
    } else {
        None
    }
}

pub fn get_dark_sky_api_key() -> String {
    app_value_of("dark-sky-api-key").unwrap()
}

pub fn get_google_api_key() -> String {
    app_value_of("google-api-key").unwrap()
}

pub fn get_http_binding() -> String {
    let address = app_value_of("address").unwrap();
    let port = app_value_of("port").unwrap();
    format!("{}:{}", &address, &port)
}

pub fn get_training_file() -> String {
    app_value_of("training").unwrap()
}