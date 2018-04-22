extern crate actix;
extern crate actix_web;
extern crate futures;
#[macro_use] extern crate clap;
#[macro_use] extern crate json;

mod chat;
mod cli;
mod weather;
mod web;

use cli::get_http_binding;
use web::bind_http_and_run;

fn main() {
    // Figure out what address the web server binds to
    let http_binding = get_http_binding();

    // Sprinkle some logging
    println!("telemacher binding to http://{}\n[press CTRL + C to stop]", &http_binding);

    // Load the web server and wait for CTRL + C or SIGTERM
    bind_http_and_run(&http_binding);
}