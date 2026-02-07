use std::path::{Path, PathBuf};

pub struct Config {
    pub reorder: bool,
    pub input_path: PathBuf,
    pub packet_offset: usize,
}

impl Config {
    fn new() -> Self {
        Self {
            reorder: false,
            input_path: PathBuf::from("app/assets/mdf-kospi200.20110216-0.pcap"),
            packet_offset: 42,
        }
    }

    fn set_reorder(&mut self, val: bool) {
        self.reorder = val;
    }

    fn set_input_path(&mut self, path: PathBuf) {
        self.input_path = path;
    }

    fn is_pcap_file(arg: &str) -> bool {
        Path::new(arg).extension().is_some_and(|ext| ext == "pcap")
    }

    pub fn build<I: Iterator<Item = String>>(args: I) -> Self {
        let mut config = Self::new();
        for arg in args.skip(1) {
            match arg.as_str() {
                "-r" => config.set_reorder(true),
                path if !path.starts_with('-') && Self::is_pcap_file(path) => {
                    config.set_input_path(PathBuf::from(path))
                }
                _ => eprintln!("Unknown argument: '{}'", arg),
            }
        }
        config
    }
}
