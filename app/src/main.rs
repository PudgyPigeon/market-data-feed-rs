mod config;
mod debug;
mod processor;
mod protocol;
mod quote;
mod strategy;
use config::Config;
use mimalloc::MiMalloc;
use pcap::{Capture, Offline};
use std::env;
use strategy::{ProcessingStrategy, StrategyType};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;


fn loop_packets<Strategy: ProcessingStrategy>(
    cap: &mut Capture<Offline>,
    processor: &mut processor::Processor,
    strategy: Strategy,
) {
    let mut sequence_counter: u64 = 0;
    while let Ok(packet) = cap.next_packet() {
        processor.process_packet(&strategy, &packet, sequence_counter);
        sequence_counter += 1;
    }
}

fn run(config: Config) {
    let mut cap = Capture::from_file(&config.input_path).unwrap();
    let mut processor = processor::Processor::new(config.quote_layout, config.packet_offset);

    match config.strategy {
        StrategyType::ImmediateMode(s) => loop_packets(&mut cap, &mut processor, s),
        StrategyType::ReorderMode(s) => loop_packets(&mut cap, &mut processor, s),
    }

    processor.close()
}

fn main() {
    run(Config::build_from_args(env::args()));
}
