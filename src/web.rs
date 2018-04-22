use actix_web::{server, App, HttpRequest, HttpResponse, HttpMessage};
use actix_web::http::Method;
use actix_web::multipart::MultipartItem;

use futures::{Future, Stream};

// TODO: Investigate Actix web way of doing this -> defaults to 256kb which may be fine: const INCOMING_HTTP_BODY_MAX: usize = 1024;
const SHUTDOWN_TIMEOUT_SECS: u16 = 1;

pub fn bind_http_and_run(http_binding: &str) {
    server::new(|| App::new()
            .resource("/chat/messages", |r| {
                r.method(Method::POST)
                    .with(chat_messages);
            }))
        .bind(http_binding).expect(&format!("could not bind to {}", http_binding))
        .shutdown_timeout(SHUTDOWN_TIMEOUT_SECS)
        .run();
}

fn chat_messages(mut request: HttpRequest) -> HttpResponse {
    // Collect the input parts into a vector
    let parts = match request.multipart().collect().wait() {
        Ok(parts) => parts,
        _ => return HttpResponse::BadRequest().finish(),
    };

    // Parse out common fields
    let action = find_part(&parts, "action");
    let user_id = find_part(&parts, "user_id");

    // Create a new chat request to handle this action
    match &action {
        &Some(ref action) if action == "join" => (),
        &Some(ref action) if action == "message" => (),
        _ => return HttpResponse::BadRequest().finish(),
    };

    HttpResponse::Ok().finish()
}

fn find_part(parts: &[MultipartItem<HttpRequest>], name: &str) -> Option<String> {
    for part in parts {
        match part {
            &MultipartItem::Field(ref item) if item.headers().contains_key(name) => {
                //let val = item.headers()[name].to_str();
                //match val {
                    //Ok(e)
                //}

                return Some("test".to_string());
            },
            _ => (),
        }
    }

    None
}