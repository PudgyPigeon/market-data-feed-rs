use crate::protocol::{KOSPI_LAYOUT, QuoteLayout};
use crate::strategy::{ImmediateMode, ReorderMode, StrategyType};
use std::path::{Path, PathBuf};

pub struct Config {
    pub strategy: StrategyType,
    pub input_path: PathBuf,
    pub packet_offset: usize,
    pub quote_layout: &'static QuoteLayout,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            strategy: StrategyType::ImmediateMode(ImmediateMode),
            input_path: PathBuf::from("app/assets/mdf-kospi200.20110216-0.pcap"),
            packet_offset: 42,
            quote_layout: &KOSPI_LAYOUT,
        }
    }
}

impl Config {
    pub fn strategy(mut self, val: StrategyType) -> Self {
        self.strategy = val;
        self
    }

    pub fn input_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.input_path = path.into();
        self
    }

    fn is_pcap_file(path: &str) -> bool {
        Path::new(path).extension().is_some_and(|ext| ext == "pcap")
    }

    pub fn build_from_args<I>(args: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let mut config = Self::default();
        let mut args = args.into_iter().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-r" | "--reorder" => {
                    config = config.strategy(StrategyType::ReorderMode(ReorderMode))
                }
                "-l" | "--layout" => if let Some(_layout_name) = args.next() {},
                path if !path.starts_with('-') && Self::is_pcap_file(path) => {
                    config = config.input_path(path);
                }
                _ => eprintln!("Warning: Unknown argument '{}'", arg),
            }
        }
        config
    }
}
