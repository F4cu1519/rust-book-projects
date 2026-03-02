use std::env;
use std::process;

use minigrep3::Config;
use minigrep3::run;

fn main() {
    // Pasamos env::args() directamente: ya no necesitamos collect()
    // ni el Vec<String> intermedio. Config::build toma ownership
    // del iterador y lo consume internamente.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}