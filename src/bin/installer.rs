use maya_scientific_computing_tools_installer::{Config, Installer};

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Init Error: {}", err);
        process::exit(1);
    });
    let installer = Installer::new(config);

    println!("{:#?}", installer);
    installer.install();
}
