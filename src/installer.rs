pub mod config;

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use config::Config;

type InstallResult<T> = Result<T, &'static str>;

#[derive(Debug)]
pub struct Installer {
    pub config: Config
}

impl Installer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn install(&self) -> InstallResult<()> {
        self.add_mayapy_exec_to_path()?;

        let mut install_bat_file = PathBuf::from(&self.config.tools_path);
        install_bat_file.push(&self.config.pkg_name);
        install_bat_file.push("install.bat");

        if !install_bat_file.is_file() {
            return Err("Could not find the install.bat file for this PKG.");
        }

        let output = match Command::new(install_bat_file).output() {
            Ok(output) => output,
            Err(_) => return Err("Error executing install.bat.")
        };

        let output_error = String::from_utf8_lossy(&output.stderr);

        if !output_error.is_empty() {
            eprintln!("{}", output_error);
            return Err("Error during installation. (If it is pip's warning can be ignored)");
        }

        Ok(())
    }

    fn add_mayapy_exec_to_path(&self) -> InstallResult<()> {
        // ERROR: Not available in release mode
        // let path_var = env::vars().collect::<HashMap<String, String>>();
        // let path_var = match path_var.get("PATH") {
        //     Some(var) => {
        //         format!("{};{}", var, &self.config.maya_bin_dir.to_str().unwrap())
        //     }
        //     None => return Err("Can't get path var.")
        // };

        env::set_var("PATH", format!("{}", &self.config.maya_bin_dir.to_str().unwrap()));

        Ok(())
    }
}
