mod config;
mod debug;
mod processor;
mod protocol;
mod quote;
use config::Config;
use pcap::Capture;
use std::env;
use std::time::Instant;

fn run(config: Config) {
    let mut cap = Capture::from_file(&config.input_path).unwrap();
    let mut processor =
        processor::Processor::new(config.reorder, config.quote_layout, config.packet_offset);
    let mut sequence_counter: u64 = 0;

    loop {
        match cap.next_packet() {
            Ok(packet) => {
                processor.process_packet(packet.data, sequence_counter);
                sequence_counter += 1;
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
    processor.close()
}

fn main() {
    let start = Instant::now();
    run(Config::build(env::args()));
    eprintln!("Total execution time: {:?}", start.elapsed());
}
