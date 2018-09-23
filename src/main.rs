#[macro_use]
extern crate log;
extern crate clap;
extern crate pretty_env_logger;

// Import self
extern crate fuchsine;

use std::env;
use std::process;
use clap::{Arg, App, SubCommand};

fn main() {
    // Use clap for command line support
    let matches = App::new("Fuchsine")
        .version("0.1.0")
        .author("KayMW <redl0tus@noreply.github.com>")
        .about(" Fuchsine Uncomplicated Customizable HTTP Server Index & Navigtion Engine (Rust ver.).")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file.")
            .takes_value(true))
        .subcommand(SubCommand::with_name("run")
            .about("Start the server"))
        .get_matches();
    // Initialize logger
    if let Err(_) = env::var("FUCHSINE_LOG") {
        env::set_var("FUCHSINE_LOG", "info");
    }
    pretty_env_logger::init_custom_env("FUCHSINE_LOG");
    info!("Start entry sequence.");
    // Find config file from command line option
    let config_filename: String = match matches.value_of("config") {
        Some(filename) => filename.to_string(),
        None => String::from("config.toml"),
    };
    // When the subcommand is `run`
    if let Some(_) = matches.subcommand_matches("run") {
        // Run the main program
        info!("Initializing LCL ionization.");
        debug!("(Got config filename: {})", config_filename);
        if let Err(e) = fuchsine::run(config_filename) {
            error!("Application error: {}", e);
            process::exit(1);
        }
        process::exit(0);
    }
    info!("No command specified, use --help for more information");
    process::exit(0);
}
