use crate::buffer::ReorderBuffer;
use crate::formatter::Formatter;
use crate::parse_time::window_has_passed;
use crate::protocol::QuoteLayout;
use crate::quote::Quote;
use crate::strategy::ProcessingStrategy;
use std::io::{self, BufWriter, Write};

pub struct Processor {
    quote_layout: &'static QuoteLayout,
    packet_offset: usize,
    pub buffer: ReorderBuffer,
    pub formatter: Formatter,
    pub writer: BufWriter<io::StdoutLock<'static>>,
}

impl Processor {
    pub fn new(quote_layout: &'static QuoteLayout, packet_offset: usize) -> Self {
        Self {
            quote_layout,
            packet_offset,
            buffer: ReorderBuffer::new(100_000),
            formatter: Formatter::new(),
            writer: BufWriter::with_capacity(1024 * 1024, io::stdout().lock()),
        }
    }

    #[inline(always)]
    pub fn process_packet<S: ProcessingStrategy>(
        &mut self,
        strategy: &S,
        packet: &pcap::Packet,
        sequence_counter: u64,
    ) {
        let Some(payload) = packet.data.get(self.packet_offset..) else { return };
        let Some(quote) = Quote::from_bytes(payload, self.quote_layout, packet.header.ts) else {
            return;
        };
        strategy.handle(self, quote, sequence_counter);
    }

    #[inline(always)]
    pub fn drain_expired(&mut self) {
        while let Some(top) = self.buffer.heap.peek() {
            if window_has_passed(top.accept_time, self.buffer.max_time_seen) {
                let q = self.buffer.heap.pop().unwrap();
                let bytes = self.formatter.format_owned(&q);
                let _ = self.writer.write_all(bytes);
            } else {
                break;
            }
        }
    }

    pub fn close(&mut self) {
        while let Some(q) = self.buffer.heap.pop() {
            let bytes = self.formatter.format_owned(&q);
            let _ = self.writer.write_all(bytes);
        }
        let _ = self.writer.flush();
    }
}
