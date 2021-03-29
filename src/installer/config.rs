use std::convert::From;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub maya_location: String,
    pub pkg_name: String,
}

impl Config {
    pub fn new(maya_location: String, pkg_name: String) -> Self {
        let config = Self { maya_location, pkg_name };
        config.check();
        config
    }

    fn check(&self) {
        self.check_args();
        self.check_dependent_pkgs();
    }

    fn check_args(&self) {
        // TODO: 检查 maya 路径与 pkg 名称是否正确
    }

    fn check_dependent_pkgs(&self) {
        // TODO: 检查当前安装的库依赖库是否齐全
    }
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

        Self::new(maya_location, pkg_name)
    }
}
