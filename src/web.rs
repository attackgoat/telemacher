use std::io::Error;
use std::str;

use futures::future;

use num_cpus;

use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

// Panics
const PANIC_UNACCEPTABLE_HTTP_BINDING: &'static str = "Unacceptable http binding";

// Headers
const HEADER_BOUNDARY: &'static str = "boundary";
const HEADER_CONTENT_TYPE: &'static str = "Content-Type";

// Methods
const METHOD_POST: &'static str = "POST";

// Mime types
const MIME_TYPE_APPLICATION_JSON: &'static str = "application/json";
const MIME_TYPE_MULTIPART_FORM_DATA: &'static str = "multipart/form-data";

// Status codes
const STATUS_CODE_BAD_REQUEST_ALPHA: &'static str = "bad request";
const STATUS_CODE_BAD_REQUEST_NUMERIC: u32 = 400;
const STATUS_CODE_NOT_FOUND_ALPHA: &'static str = "not found";
const STATUS_CODE_NOT_FOUND_NUMERIC: u32 = 404;

// Routes
const ROUTE_CHAT_MESSAGES: &'static str = "/chat/messages";

fn get_header(request: &Request, key: &str) -> Option<String> {
    let key = key.to_lowercase();
    for (header_key, header_val) in request.headers() {
        if key == header_key.to_lowercase().trim() {
            if let Ok(val) = str::from_utf8(header_val) {
                return Some(val.to_owned());
            }
        }
    }

    None
}

pub struct Router;

impl Router {
    pub fn serve_forever(http_binding: &str) {
        // Parse input string into the tokio address type
        let http_binding = http_binding.parse().expect(PANIC_UNACCEPTABLE_HTTP_BINDING);

        // The new webserver will use a thread per core
        let mut server = TcpServer::new(Http, http_binding);
        server.threads(num_cpus::get());
        server.serve(|| Ok(Router));
    }

    fn chat_messages(&self, request: &Request) -> Response {
        // Get any required headers
        let content_type = get_header(request, HEADER_CONTENT_TYPE);

        // Sanity check: Must have Content-Type
        if let None = content_type {
            return Self::bad_request();
        }

        // This will always succeed because it's not-none
        let content_type = content_type.unwrap();
        let content_type_parts: Vec<&str> = content_type.split(';').collect();

        // Sanity check: Content-Type must be form data
        if content_type_parts.len() < 2 || content_type_parts[0].to_lowercase().trim() != MIME_TYPE_MULTIPART_FORM_DATA {
            return Self::bad_request();
        }

        // This is the header we're parsing to find the boundary:
        // Content-Type: multipart/form-data; boundary=----WebKitFormBoundaryhtiHGRJWxqAGATpt
        let boundary_parts: Vec<&str> = content_type_parts[1].split('=').collect();

        // Sanity check: Content-Type must have boundary
        if boundary_parts.len() != 2 || boundary_parts[0].to_lowercase().trim() != HEADER_BOUNDARY {
            return Self::bad_request();
        }

        // This will always succeed because we checked above
        let boundary = boundary_parts[1].trim();

        println!("<{}>", boundary);
    

        let mut response = Response::new();



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
            ROUTE_CHAT_MESSAGES if request.method().to_uppercase().trim() == METHOD_POST => self.chat_messages(&request),
            _ => Self::not_found(),
        })
    }
}
