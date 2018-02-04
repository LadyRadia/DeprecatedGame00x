#[macro_use]
extern crate log;
extern crate log4rs;
pub extern crate piston;
pub extern crate graphics;
pub extern crate glutin_window;
pub extern crate opengl_graphics;



use std::env;
use std::path;

mod controller;
mod model;
mod view;

use controller::Controller;

fn main() {
    initialize_logging();

    match Controller::run_client_application() {
        Ok(done) => info!("Exiting Chi"),
        Err(err) => error!("Error running Chi: {}", err)
    }    
    info!("Main Thread exiting OK");
}

fn initialize_logging() {
    log4rs::init_file("conf/log4rs.yml", Default::default()).expect("Failed to initialize logging; hard aborting.");
    debug!("Initialized logging from file.");
}