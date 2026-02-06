use pcap::Capture;
use std::path::{Path, PathBuf};
use std::{env, str};

struct Config {
    reorder: bool,
    input_path: PathBuf,
}

impl Config {
    fn new() -> Self {
        Self {
            reorder: false,
            input_path: PathBuf::from("app/assets/mdf-kospi200.20110216-0.pcap"),
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

    fn parse_input_args<I: Iterator<Item = String>>(mut self, args: I) -> Self {
        for arg in args.skip(1) {
            match arg.as_str() {
                "-r" => self.set_reorder(true),
                path if !path.starts_with('-') && Self::is_pcap_file(path) => {
                    self.set_input_path(PathBuf::from(path))
                }
                _ => eprintln!("Unknown argument: '{}'", arg),
            }
        }
        self
    }
}

#[derive(Debug)]
struct Quote<'packet> {
    issue_code: &'packet str,
    issue_seq_no: &'packet str,
    data_type: &'packet str,
    info_type: &'packet str,
    market_type: &'packet str,
    accept_time: &'packet str,
    total_bid_vol: &'packet str,
    total_ask_vol: &'packet str,
    best_bids: [PriceQty<'packet>; 5],
    best_asks: [PriceQty<'packet>; 5],
}

#[derive(Debug)]
struct PriceQty<'packet> {
    price: &'packet str,
    qty: &'packet str,
}

fn run(config: Config) {
    let mut cap = match Capture::from_file(&config.input_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error opening pcap file: {}", e);
            return;
        }
    };
    println!("Successfully opened: {:?}", config.input_path);
    while let Ok(packet) = cap.next_packet() {
        println!("Received packet with length: {}", packet.header.len);
        println!("Raw Data: {:?}", packet.data)
    }
}

fn main() {
    let config = Config::new().parse_input_args(env::args());
    run(config)
}
