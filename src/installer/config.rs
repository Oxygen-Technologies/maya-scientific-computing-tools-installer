use std::env;
use std::path::PathBuf;

type ConfigResult<T> = Result<T, &'static str>;

#[derive(Debug)]
pub struct Config {
    pub maya_location_path: PathBuf,
    pub maya_bin_dir: PathBuf,
    pub tools_path: PathBuf,
    pub pkg_name: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> ConfigResult<Config> {
        let (maya_location_path, pkg_name) = match (args.get(1), args.get(2)) {
            (Some(maya_location), Some(pkg_name)) => (maya_location.clone(), pkg_name.clone()),
            (Some(_), None) => return Err("Need pkg name."),
            _ => {
                return Err("Need maya location path.");
            }
        };

        let maya_location_path = PathBuf::from(maya_location_path);
        let mut maya_bin_dir = maya_location_path.clone();
        maya_bin_dir.push("bin");

        let tools_path = match env::current_dir() {
            Ok(mut dir) => {
                dir.push("maya-scientific-computing-tools");
                if !dir.is_dir() {
                    return Err("Need maya scientific computing tools.")
                }
                dir
            }
            Err(_) => return Err("Get current dir error.")
        };

        let config = Self {
            maya_location_path,
            maya_bin_dir,
            tools_path,
            pkg_name,
        };
        config.check()?;

        Ok(config)
    }

    fn check(&self) -> ConfigResult<()> {
        self.check_mayapy_exec_path()?;
        self.check_dependent_pkgs()?;

        Ok(())
    }

    fn check_mayapy_exec_path(&self) -> ConfigResult<()> {
        if !self.maya_bin_dir.is_dir() {
            return Err("This is not a valid maya location path.");
        }

        Ok(())
    }

    fn check_dependent_pkgs(&self) -> ConfigResult<()> {
        // TODO: 检查当前安装的库依赖库是否齐全

        Ok(())
    }
}
