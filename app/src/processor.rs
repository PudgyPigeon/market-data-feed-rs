use crate::protocol::QuoteLayout;
use crate::quote::{Quote, QuoteOwned};
use std::collections::BinaryHeap;
use std::io::{self, BufWriter, Write};

pub struct Processor {
    reorder: bool,
    quote_layout: &'static QuoteLayout,
    packet_offset: usize,
    heap: BinaryHeap<QuoteOwned>,
    max_time_seen: u64,
    writer: BufWriter<io::StdoutLock<'static>>,
}

impl Processor {
    pub fn new(reorder: bool, quote_layout: &'static QuoteLayout, packet_offset: usize) -> Self {
        Self {
            reorder,
            quote_layout,
            packet_offset,
            // Capacity of 1024 prevents re-allocations during typical 3-sec bursts
            heap: BinaryHeap::with_capacity(10_000), //1024
            max_time_seen: 0,
            writer: BufWriter::new(io::stdout().lock()),
        }
    }

    /// Processes a single market‑data packet.
    ///
    /// # Arguments
    /// - `data`: Raw packet bytes from the capture file.
    /// - `sequence_counter`: Monotonic sequence number assigned by the caller.
    ///
    /// # Behavior
    /// - Drops malformed packets silently.
    /// - Prints immediately in non‑reorder mode.
    /// - Buffers and reorders quotes in reorder mode using a 3‑second window.
    ///
    /// # Returns
    /// This function returns `()` and never propagates errors.
    ///
    /// # Notes
    /// Designed for high‑throughput (HFT) packet processing where malformed
    /// packets must not interrupt the hot path.
    pub fn process_packet(&mut self, data: &[u8], sequence_counter: u64) {
        let Some(payload) = data.get(self.packet_offset..) else { return };
        let Some(quote) = Quote::from_bytes(payload, self.quote_layout) else { return };

        if !self.reorder {
            self.print_borrowed(&quote);
            return;
        }

        // update max time seen for future operations
        if quote.accept_time > self.max_time_seen {
            self.max_time_seen = quote.accept_time;
        }

        self.heap.push(quote.to_owned(sequence_counter));

        // Sliding window to account for 3sec max discrepancy between accept times
        while let Some(top_of_heap) = self.heap.peek() {
            if self.window_has_passed(top_of_heap.accept_time) {
                let quote = self.heap.pop().unwrap();
                self.print_owned(&quote);
            } else {
                break;
            }
        }
    }

    pub fn close(&mut self) {
        // Empty heap, flush, and print everything
        while let Some(quote) = self.heap.pop() {
            self.print_owned(&quote);
        }
        let _ = self.writer.flush();
    }

    fn window_has_passed(&self, packet_time: u64) -> bool {
        let packet_time_centi = self.to_centiseconds(packet_time);
        let max_time_centi = self.to_centiseconds(self.max_time_seen);
        max_time_centi.saturating_sub(packet_time_centi) >= 300 // 3 seconds = 300 centiseconds
    }

    fn to_centiseconds(&self, time: u64) -> u64 {
        let hh = time / 1_00_00_00;
        let mm = (time / 1_00_00) % 100;
        let ss = (time / 1_00) % 100;
        let uu = time & 100;
        ((hh * 3600) + (mm * 60) + ss) * 100 + uu
    }

    fn print_owned(&mut self, quote: &QuoteOwned) {
        unsafe {
            let code = std::str::from_utf8_unchecked(&quote.issue_code);
            let bid_price = std::str::from_utf8_unchecked(&quote.bids[0].0);
            let ask_price = std::str::from_utf8_unchecked(&quote.asks[0].0);
            let ask_qty = std::str::from_utf8_unchecked(&quote.asks[0].1);

            let _ = writeln!(
                self.writer,
                "[{}] Issue: {} | Best Bid: {} | Best Ask: {} | Ask Qty: {},",
                quote.accept_time, code, bid_price, ask_price, ask_qty
            );
        }
    }

    fn print_borrowed(&mut self, quote: &Quote) {
        let _ = writeln!(
            self.writer,
            "[{}] Issue: {} | Best Bid: {} | Best Ask: {} | Ask Qty: {},",
            quote.accept_time,
            quote.issue_code.trim(),
            quote.bids[0].price,
            quote.asks[0].price,
            quote.asks[0].qty
        );
    }
}
