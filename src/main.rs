mod installer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = installer::config::Config::from(&args);
    let installer = installer::Installer::new(config);

    println!("{:#?}", installer);
    installer.install();
}
