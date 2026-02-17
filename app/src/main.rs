mod config;
mod processor;
mod protocol;
mod quote;
mod strategy;
use config::Config;
use mimalloc::MiMalloc;
use pcap::Capture;
use processor::Processor;
use std::env;
use strategy::StrategyType;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    
    let config = Config::build_from_args(env::args());
    let mut cap = Capture::from_file(&config.input_path).unwrap();
    let mut processor = Processor::new(config.quote_layout, config.packet_offset);
    let mut sequence_counter: u64 = 0;

    match config.strategy {
        StrategyType::ImmediateMode(strategy) => {
            while let Ok(packet) = cap.next_packet() {
                processor.process_packet(&strategy, &packet, sequence_counter);
                sequence_counter += 1;
            }
        }
        StrategyType::ReorderMode(strategy) => {
            while let Ok(packet) = cap.next_packet() {
                processor.process_packet(&strategy, &packet, sequence_counter);
                sequence_counter += 1;
            }
        }
    }

    processor.close()
}
