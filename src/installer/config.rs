use std::convert::From;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub maya_location: String,
    pub pkg_name: String,
}

impl From<&Vec<String>> for Config {
    fn from(args: &Vec<String>) -> Self {
        let (maya_location, pkg_name) = match (args.get(1), args.get(2)) {
            (Some(maya_location), Some(pkg_name)) => (maya_location.clone(), pkg_name.clone()),
            _ => {
                eprintln!("Args Error: Need maya location path and pkg name.");
                process::exit(1);
            }
        };

        Self { maya_location, pkg_name }
    }
}
