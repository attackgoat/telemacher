use std::io::Error;

use futures::future;

use num_cpus;

use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

// Headers
const HEADER_CONTENT_TYPE: &'static str = "Content-Type";

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

    fn chat_messages() -> future::Ok<Response, Error> {
        let mut response = Response::new();
        response.header(HEADER_CONTENT_TYPE, MIME_TYPE_APPLICATION_JSON)
            .body(&"{}");
        future::ok(response)
    }

    fn bad_request() -> future::Ok<Response, Error> {
        let mut response = Response::new();
        response.status_code(STATUS_CODE_BAD_REQUEST_NUMERIC, STATUS_CODE_BAD_REQUEST_ALPHA);
        future::ok(response)
    }

    fn not_found() -> future::Ok<Response, Error> {
        let mut response = Response::new();
        response.status_code(STATUS_CODE_NOT_FOUND_NUMERIC, STATUS_CODE_NOT_FOUND_ALPHA);
        future::ok(response)
    }
}

impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = future::Ok<Response, Error>;

    fn call(&self, request: Request) -> Self::Future {
        match request.path() {
            ROUTE_CHAT_MESSAGES => Self::chat_messages(),
            _ => Self::not_found(),
        }
    }
}