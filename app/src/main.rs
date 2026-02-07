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
use std::time::Instant;
use strategy::ProcessingStrategy;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// The 'TYPE' Strategy must implement the ProcessingStrategy 'TRAIT'
// Sequence counter for time ordering
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

    if config.reorder {
        loop_packets(&mut cap, &mut processor, strategy::ReorderMode);
    } else {
        loop_packets(&mut cap, &mut processor, strategy::ImmediateMode);
    }

    processor.close()
}

fn main() {
    let start = Instant::now();
    run(Config::build(env::args()));
    eprintln!("Total execution time: {:?}", start.elapsed());
}
