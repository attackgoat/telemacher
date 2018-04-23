#[macro_use] extern crate clap;
extern crate futures;
#[macro_use] extern crate json;
extern crate lru_cache;
extern crate multipart;
extern crate num_cpus;
extern crate reqwest;
extern crate snips_nlu_lib;
extern crate snips_nlu_ontology;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;
extern crate urlencoding;

mod cli;
mod dark_sky;
mod google;
mod harris;
mod web;

use cli::get_http_binding;
use web::Router;

fn main() {
    // Figure out what address the web server binds to
    let http_binding = get_http_binding();

    // Sprinkle some logging
    println!("telemacher binding to http://{}\n[press CTRL + C to stop]", &http_binding);

    // Load the web server and wait for CTRL + C or SIGTERM
    Router::serve_forever(&http_binding);
}