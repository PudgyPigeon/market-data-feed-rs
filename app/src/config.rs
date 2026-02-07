use crate::protocol::{KOSPI_LAYOUT, QuoteLayout};
use std::path::{Path, PathBuf};

pub struct Config {
    pub reorder: bool,
    pub input_path: PathBuf,
    pub packet_offset: usize,
    pub quote_layout: &'static QuoteLayout,
}

impl Config {
    fn new() -> Self {
        Self {
            reorder: false,
            input_path: PathBuf::from("app/assets/mdf-kospi200.20110216-0.pcap"),
            packet_offset: 42,
            quote_layout: &KOSPI_LAYOUT,
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
                // TODO: add a new flag here for specifying layout to load in
                path if !path.starts_with('-') && Self::is_pcap_file(path) => {
                    config.set_input_path(PathBuf::from(path))
                }
                _ => eprintln!("Unknown argument: '{}'", arg),
            }
        }
        config
    }
    // For future implementation of more quote layouts
    // This is unused for now since we only want KOSPI atm
    // We would put that match statement below with flag --spec or --layout or something
    // fn set_quote_layout(&mut self, quote_layout: &'static QuoteLayout) {
    //     self.quote_layout = quote_layout;
    // }
}
