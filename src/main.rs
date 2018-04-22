extern crate actix_web as web;

#[macro_use]
extern crate clap;

//#[macro_use]
extern crate json;

use std::cell::RefCell;
use std::io::Read;
use std::str;

use web::{server, App as WebApp, HttpResponse, HttpRequest};
use web::http::{ContentEncoding, Method};

const INCOMING_HTTP_BODY_MAX: usize = 1024;

fn chat_messages(mut request: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("")
}

fn main() {
    // Load the command-line-argument-parser (CLAP) library
    let cli = load_yaml!("../cli.yml");
    let clap = clap::App::from_yaml(&cli).get_matches();

    // Figure out what address the web server binds to
    let address = clap.value_of("address").unwrap();
    let port = clap.value_of("port").unwrap();
    let http_binding = format!("{}:{}", &address, &port);
    println!("telemacher binding to http://{}\n[press CTRL + C to stop]", &http_binding);

    // Load the web server and wait for CTRL + C or SIGTERM
    server::new(|| WebApp::new()
            .default_encoding(ContentEncoding::Auto)
            .route("/chat/messages", Method::POST, chat_messages))
        .bind(&http_binding).expect(&format!("could not bind to {}", &http_binding))
        .run();
}

enum Request {
    Join(Join),
    Message(Message),
}

struct Join {
    name: String,
    user_id: u64,
}

struct Message {
    text: String,
    user_id: u64,
}

struct Response {
    messages: Vec<String>,
}