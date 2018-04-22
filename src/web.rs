use std::io::Error;

use futures::future;

use num_cpus;

use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

// Headers
const HEADER_CONTENT_TYPE: &'static str = "Content-Type";

// Methods
const METHOD_POST: &'static str = "POST";

// Mime types
const MIME_TYPE_APPLICATION_JSON: &'static str = "application/json";

// Status codes
const STATUS_CODE_BAD_REQUEST_ALPHA: &'static str = "bad request";
const STATUS_CODE_BAD_REQUEST_NUMERIC: u32 = 400;
const STATUS_CODE_NOT_FOUND_ALPHA: &'static str = "not found";
const STATUS_CODE_NOT_FOUND_NUMERIC: u32 = 404;

// Routes
const ROUTE_CHAT_MESSAGES: &'static str = "/chat/messages";

pub struct Router;

impl Router {
    pub fn serve_forever(http_binding: &str) {
        // Parse input string into the tokio address type
        let http_binding = http_binding.parse().expect("Unacceptable http binding");

        // The new webserver will use a thread per core
        let mut server = TcpServer::new(Http, http_binding);
        server.threads(num_cpus::get());
        server.serve(|| Ok(Router));
    }

    fn chat_messages(&self, request: &Request) -> Response {
        let mut response = Response::new();



        println!("Hello, world!");

        response.header(HEADER_CONTENT_TYPE, MIME_TYPE_APPLICATION_JSON)
            .body(&"{}");
        response
    }

    fn bad_request() -> Response {
        let mut response = Response::new();
        response.status_code(STATUS_CODE_BAD_REQUEST_NUMERIC, STATUS_CODE_BAD_REQUEST_ALPHA);
        response
    }

    fn not_found() -> Response {
        let mut response = Response::new();
        response.status_code(STATUS_CODE_NOT_FOUND_NUMERIC, STATUS_CODE_NOT_FOUND_ALPHA);
        response
    }
}

impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = future::Ok<Response, Error>;

    fn call(&self, request: Request) -> Self::Future {
        future::ok(match request.path() {
            ROUTE_CHAT_MESSAGES if request.method() == METHOD_POST => self.chat_messages(&request),
            _ => Self::not_found(),
        })
    }
}
