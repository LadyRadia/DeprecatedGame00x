#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;
use std::path;

mod controller;
mod model;
mod view;

use controller::Controller;

fn main() {
    initialize_logging();
    let mut controller = Controller::initialize_controller();
    match controller.run() {
        Ok(done) => info!("Exiting Chi"),
        Err(err) => error!("Error running Chi: {}", err)
    }    
    info!("Main Thread exiting OK");
}

fn initialize_logging() {
    log4rs::init_file("conf/log4rs.yml", Default::default()).expect("Failed to initialize logging; hard aborting.");
    debug!("Initialized logging from file.");
}