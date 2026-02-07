mod config;
mod debug;
mod processor;
mod protocol;
mod quote;
use config::Config;
use pcap::Capture;
use std::env;

fn run(config: Config) {
    debug_init!(count)

    let mut cap = Capture::from_file(&config.input_path)
        .expect("ERROR: Could not open pcap file or invalid path");

    loop {
        match cap.next_packet() {
            Ok(packet) => {
                processor::process_packet(&packet, config.packet_offset);
                debug_break!(count, 100);
            }
            Err(pcap::Error::NoMorePackets) => {
                eprintln!("Reached the end of the .pcap file. Breaking loop.");
                break;
            }
            Err(e) => {
                eprintln!("Error reading next packet: {}", e);
                break;
            }
        }
    }
}

fn main() {
    run(Config::build(env::args()));
}
