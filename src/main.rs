mod core;
mod api;
mod cli;
mod gui;
mod database;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.contains(&"--api".to_string()) {
        api::start_server();
        return;
    }

    if args.contains(&"--cli".to_string()) {
        cli::run();
        return;
    }

    if args.contains(&"--json".to_string()) {
        cli::run_json();
        return;
    }

    if let Err(e) = gui::run() {
        eprintln!("Error al iniciar la GUI: {}", e);
        std::process::exit(1);
    }
}
