use std::io::{Error, Read};
use std::str;

use futures::future;

use multipart::server::Multipart;

use num_cpus;

use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

use super::chat::{Event, Harris, Join, Message};

// Panics
const PANIC_UNACCEPTABLE_HTTP_BINDING: &'static str = "Unacceptable http binding";

// Headers
const HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: &'static str = "Access-Control-Allow-Origin";
const HEADER_ACCESS_CONTROL_ALLOW_ORIGIN_STAR: &'static str = "*";
const HEADER_BOUNDARY: &'static str = "boundary";
const HEADER_CONTENT_TYPE: &'static str = "Content-Type";
const HEADER_ORIGIN: &'static str = "Origin";

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

// Form data fields
const FORM_DATA_ACTION: &'static str = "action";
const FORM_DATA_NAME: &'static str = "name";
const FORM_DATA_TEXT: &'static str = "text";
const FORM_DATA_USER_ID: &'static str = "user_id";

// Actions
const ACTION_JOIN: &'static str = "join";
const ACTION_MESSAGE: &'static str = "message";

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

fn try_get_header(request: &Request, key: &str) -> Option<String> {
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

fn try_get_multipart(request: &Request) -> Option<Multipart<&[u8]>> {
    // Get any required headers
    let content_type = try_get_header(request, HEADER_CONTENT_TYPE);

    // Sanity check: Must have Content-Type
    if let None = content_type {
        return None;
    }

    // This will always succeed because it's not-none
    let content_type = content_type.unwrap();
    let content_type_parts: Vec<&str> = content_type.split(';').collect();

    // Sanity check: Content-Type must be form data
    if content_type_parts.len() < 2 || content_type_parts[0].to_lowercase().trim() != MIME_TYPE_MULTIPART_FORM_DATA {
        return None;
    }

    // This is the header we're parsing to find the boundary:
    // Content-Type: multipart/form-data; boundary=----WebKitFormBoundaryhtiHGRJWxqAGATpt
    let boundary_parts: Vec<&str> = content_type_parts[1].split('=').collect();

    // Sanity check: Content-Type must have boundary
    if boundary_parts.len() != 2 || boundary_parts[0].to_lowercase().trim() != HEADER_BOUNDARY {
        return None;
    }

    // This will always succeed because we checked above
    let boundary = boundary_parts[1].trim();
    Some(Multipart::with_body(request.body(), boundary))
}

fn try_parse_utf8<R: Read>(data: R) -> Option<String> {
    let mut buf = vec![];
    for byte in data.bytes() {
        if let Ok(byte) = byte {
            buf.push(byte);
        } else {
            return None;
        }
    }

    if let Ok(s) = str::from_utf8(&buf) {
        Some(s.to_owned())
    } else {
        None
    }
}

pub struct Router {
    harris: Harris,
}

impl Router {
    pub fn serve_forever(http_binding: &str) {
        // Parse input string into the tokio address type
        let http_binding = http_binding.parse().expect(PANIC_UNACCEPTABLE_HTTP_BINDING);

        // The new webserver will use a thread per core
        let mut server = TcpServer::new(Http, http_binding);
        server.threads(num_cpus::get());
        server.serve(|| Ok(Router::default()));
    }

    fn chat_messages(&self, request: &Request) -> Response {
        // Sanity check: Must have multipart data
        let multipart = try_get_multipart(request);
        if let None = multipart {
            return bad_request();
        }

        // Parse out the fields from all requests
        let mut action = None;
        let mut user_id = None;
        let mut name = None;
        let mut text = None;
        let mut multipart = multipart.unwrap();
        let iter = multipart.foreach_entry(|e| {
            match e.headers.name.trim().to_lowercase().as_ref() {
                FORM_DATA_ACTION => action = try_parse_utf8(e.data),
                FORM_DATA_NAME => name = try_parse_utf8(e.data),
                FORM_DATA_TEXT => text = try_parse_utf8(e.data),
                FORM_DATA_USER_ID => user_id = try_parse_utf8(e.data),
                _ => (),
            }
        });

        // Sanity check: We should have action and user_id
        if iter.is_err() || action.is_none() || user_id.is_none() {
            return bad_request();
        }

        // Sanity check: user_id should be numeric
        let user_id: Result<u64, _> = user_id.unwrap().trim().parse();
        if let Err(_) = user_id {
            return bad_request();
        }

        // Parse the correct message type
        let msg = match action.unwrap().trim().to_lowercase().as_ref() {
            ACTION_JOIN => {
                // Sanity check: We should have name
                if name.is_none() {
                    return bad_request();
                }

                Event::Join(Join::new(user_id.unwrap(), name.unwrap()))
            },
            ACTION_MESSAGE => {
                // Sanity check: We should have text
                if text.is_none() {
                    return bad_request();
                }

                Event::Message(Message::new(user_id.unwrap(), text.unwrap()))
            },
            _ => return bad_request(),
        };

        // Process the chat logic and produce a one-liner response
        let chat_response = self.harris.respond(&msg);

        // Respond to the client using json
        let mut response = Response::new();
        response.header(HEADER_CONTENT_TYPE, MIME_TYPE_APPLICATION_JSON);
        response.body(&object!{

            "messages" => array![object!{
                "type" => "text",
                "text" => chat_response,
            }],
        }.dump());

        // See if this request contains a CORS header and include a response if so
        // TODO: A real application might limit the acceptable hosts via config file, etc
        if try_get_header(request, HEADER_ORIGIN).is_some() {
            response.header(HEADER_ACCESS_CONTROL_ALLOW_ORIGIN, HEADER_ACCESS_CONTROL_ALLOW_ORIGIN_STAR);
        }

        response
    }
}

impl Default for Router {
    fn default() -> Self {
        Self {
            harris: Default::default()
        }
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
            _ => not_found(),
        })
    }
}