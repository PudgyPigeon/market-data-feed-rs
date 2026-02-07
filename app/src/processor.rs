use crate::protocol::KOSPI_LAYOUT;
use crate::quote::{Quote, QuoteOwned};
use std::collections::BinaryHeap;
use std::fmt::Binary;
use std::io::{self, BufWriter, Write};

// fn parse_quote<'packet>(
//     packet: &pcap::Packet<'packet>,
//     packet_offset: usize,
// ) -> Option<Quote<'packet>> {
//     let payload = packet.data.get(packet_offset..)?;
//     Quote::from_bytes(payload, &KOSPI_LAYOUT)
// }

// pub fn process_packet(packet: &pcap::Packet, packet_offset: usize, _sequence_counter: u64) {
//     let Some(quote) = parse_quote(packet, packet_offset) else { return };
//     println!(
//         "[{}] Issue: {} | Best Bid: {} | Best Ask: {} | Ask Qty: {},",
//         quote.accept_time,
//         quote.issue_code.trim(),
//         quote.bids[0].price,
//         quote.asks[0].price,
//         quote.asks[0].qty,
//     );
// }

pub struct Processor {
    reorder: bool,
    heap: BinaryHeap<QuoteOwned>,
    max_time_seen: u64,
    writer: BufWriter<io::StdoutLock<'static>>,
}

impl Processor {
    pub fn new(reorder: bool) -> Self {
        Self {
            reorder,
            heap: BinaryHeap::new(),
            max_time_seen: 0,
            writer: BufWriter::new(io::stdout().lock()),
        }
    }
}
