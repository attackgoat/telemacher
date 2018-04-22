extern crate actix_web as web;

#[macro_use]
extern crate clap;

use clap::App as ClapApp;

use web::{server, App as WebApp, Error as WebError, HttpResponse, HttpRequest, Responder};
use web::http::Method as HttpMethod;

fn chat_messages(req: HttpRequest) -> Response {
    Response {
        messages: vec![],
    }
}

fn main() {
    // Load the command-line-argument-parser (CLAP) library
    let cli_yaml = load_yaml!("../cli.yaml");
    let clap = ClapApp::from_yaml(cli_yaml).get_matches();

    // Load the web server and wait for CTRL + C or SIGTERM
    let address = clap.value_of("address").unwrap();
    let port = clap.value_of("port").unwrap();
    let http_binding = format!("{}:{}", address, port);
    server::new(|| WebApp::new()
            .route("/chat/messages", HttpMethod::POST, chat_messages))
        .bind(&http_binding).expect(&format!("Cannot bind to {}", &http_binding))
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

impl Responder for Response {
    type Item = HttpResponse;
    type Error = WebError;

    fn respond_to(self, _request: HttpRequest) -> Result<HttpResponse, WebError> {
        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(""))
    }
}
