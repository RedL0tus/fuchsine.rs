#![feature(extern_prelude)]

// Logging
#[macro_use]
extern crate log;

// Configuration reading support
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;

// Generating index
extern crate walkdir;

// Filesystem watching
extern crate notify;

// Web
extern crate bytes;
extern crate futures;
extern crate actix_web;

// Multithreading
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

use std::error::Error;

pub mod config;
mod index;
mod web;

pub fn run(config_filename: String) -> Result<(), Box<Error>> {
    info!("Plug depth stable at default status.");
    debug!("(Core program starting...)");
    debug!("(Reading config from {})", &config_filename);
    // Read config from given file
    let config = config::parse_config(config_filename)?;
    debug!("(Got config: {:?})", &config);
    debug!("(Start building file index...)");
    let index = Arc::new(Mutex::new(index::generate_index(config.clone().root.unwrap())?));
    info!("Boot-up voltage has cleared the threshold.");
    debug!("(Got index: {:?})", index);
    // Filesystem watcher
    // TODO: Graceful shutdown for watcher
    debug!("(Starting filesystem watcher progress...)");
    let index = Arc::clone(&index);
    let config_cloned = config.clone();
    thread::spawn(move || {
        if let Err(e) = index::start_watcher(&config_cloned.root.unwrap(), index) {
            println!("Watcher error: {:?}", e);
        }
    });
    // Web process
    let config_cloned = config.clone();
    let webserver_handle = thread::spawn(move || {
        web::start_web(&config_cloned.host.unwrap()).unwrap();
    });
    webserver_handle.join().unwrap();
    Ok(())
}