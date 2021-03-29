pub mod config;

use config::Config;

#[derive(Debug)]
pub struct Installer {
    config: Config
}

impl Installer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn install(&self) {
        self.add_mayapy_exec_to_path()
        // TODO: 安装
    }

    fn add_mayapy_exec_to_path(&self) {
        // TODO: 添加 mayapy.exe 到环境变量
    }
}
