use clap::App;

pub fn get_http_binding() -> String {
    // Load the command-line-argument-parser (CLAP) library
    let cli = load_yaml!("../cli.yml");
    let app = App::from_yaml(&cli).get_matches();
    let address = app.value_of("address").unwrap();
    let port = app.value_of("port").unwrap();
    format!("{}:{}", &address, &port)
}