use maya_scientific_computing_tools_installer::{Config, Installer};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from(&args);
    let installer = Installer::new(config);

    println!("{:#?}", installer);
    installer.install();
}
