use crate::protocol::QuoteLayout;
use crate::quote::{Quote, QuoteOwned};
use crate::strategy::ProcessingStrategy;
use std::collections::BinaryHeap;
use std::io::{self, BufWriter, Write};

pub struct Processor {
    quote_layout: &'static QuoteLayout,
    packet_offset: usize,
    heap: BinaryHeap<QuoteOwned>,
    max_time_seen: u64,
    writer: BufWriter<io::StdoutLock<'static>>,
    line_buf: Vec<u8>,
    itoa_buf: itoa::Buffer,
}

impl Processor {
    pub fn new(quote_layout: &'static QuoteLayout, packet_offset: usize) -> Self {
        Self {
            quote_layout,
            packet_offset,
            heap: BinaryHeap::with_capacity(100_000), //1024 Capacity prevents re-allocations during typical 3-sec bursts
            max_time_seen: 0,
            writer: BufWriter::with_capacity(1024 * 1024, io::stdout().lock()),
            line_buf: Vec::with_capacity(1024),
            itoa_buf: itoa::Buffer::new(),
        }
    }

    pub fn process_packet<Strategy: ProcessingStrategy>(
        &mut self,
        strategy: &Strategy,
        packet: &pcap::Packet,
        sequence_counter: u64,
    ) {
        let Some(payload) = packet.data.get(self.packet_offset..) else { return };
        let Some(quote) = Quote::from_bytes(payload, self.quote_layout, packet.header.ts) else {
            return;
        };
        strategy.handle(self, quote, sequence_counter)
    }

    pub fn close(&mut self) {
        // Empty heap, flush, and print everything
        while let Some(quote) = self.heap.pop() {
            self.print_owned(&quote);
        }
        let _ = self.writer.flush();
    }

    fn window_has_passed(&self, accept_time: u64) -> bool {
        let accept_time_centi = self.to_centiseconds(accept_time);
        let max_time_centi = self.to_centiseconds(self.max_time_seen);
        max_time_centi.saturating_sub(accept_time_centi) >= 300 // 3 seconds = 300 centiseconds
    }

    fn to_centiseconds(&self, time: u64) -> u64 {
        let hh = time / 1_00_00_00;
        let mm = (time / 1_00_00) % 100;
        let ss = (time / 1_00) % 100;
        let uu = time % 100;
        ((hh * 3600) + (mm * 60) + ss) * 100 + uu
    }

    pub fn buffer_quote(&mut self, quote: Quote, sequence_counter: u64) {
        // Updates max time seen with quote time if needed
        // Converts quote to owned object then pushes to heap
        // Uses QuoteOwned's Ord impl trait via BinaryHeap's logic
        if quote.accept_time > self.max_time_seen {
            self.max_time_seen = quote.accept_time;
        }
        self.heap.push(quote.to_owned(sequence_counter));
    }

    pub fn drain_heap(&mut self) {
        // 3 second sliding window - flushes heap if quote time is older than 3secs compared to
        // max time seen
        while let Some(top_of_heap) = self.heap.peek() {
            if self.window_has_passed(top_of_heap.accept_time) {
                let quote = self.heap.pop().unwrap();
                self.print_owned(&quote);
            } else {
                break;
            }
        }
    }

    pub fn print_owned(&mut self, q: &QuoteOwned) {
        self.line_buf.clear();
        let b = &mut self.line_buf;
        let itoa = &mut self.itoa_buf; // Make sure this is in your struct

        // 1. Timestamps - Manual placement is faster than write! macro
        b.extend_from_slice(itoa.format(q.pkt_sec).as_bytes());
        b.push(b'.');

        // For usec, if you need the leading zeros (e.g. .000123),
        // the write! macro is actually easier/safer here:
        let _ = write!(b, "{:06}", q.pkt_usec);

        b.push(b',');
        b.extend_from_slice(itoa.format(q.accept_time).as_bytes());
        b.push(b',');

        // 2. Issue Code (Fixed-size array)
        let code = &q.issue_code;
        let code_len = code.iter().position(|&x| x == b' ' || x == 0).unwrap_or(code.len());
        b.extend_from_slice(&code[..code_len]);

        // 3. The Ladder (Direct memory copies)
        for i in (0..5).rev() {
            b.push(b',');
            let (p_arr, q_arr) = &q.bids[i];

            // Qty
            let q_len = q_arr.iter().position(|&x| x == b' ' || x == 0).unwrap_or(q_arr.len());
            b.extend_from_slice(&q_arr[..q_len]);

            b.push(b',');

            // Price
            let p_len = p_arr.iter().position(|&x| x == b' ' || x == 0).unwrap_or(p_arr.len());
            b.extend_from_slice(&p_arr[..p_len]);
        }

        // ... Repeat for asks ...

        b.push(b'\n');
        let _ = self.writer.write_all(b);
    }

    pub fn print_borrowed(&mut self, q: &Quote) {
        let w = &mut self.writer;
        // Use a local scratchpad (pre-allocated in the struct for speed)
        self.line_buf.clear();
        let buf = &mut self.line_buf;

        // Format the dynamic timestamps into the scratchpad
        let _ = write!(buf, "{}.{:06},{},", q.pkt_sec, q.pkt_usec, q.accept_time);

        //  Direct memory copies for the rest (No formatting logic)
        buf.extend_from_slice(q.issue_code.as_bytes());

        for i in (0..5).rev() {
            buf.push(b',');
            buf.extend_from_slice(q.bids[i].qty.as_bytes());
            buf.push(b',');
            buf.extend_from_slice(q.bids[i].price.as_bytes());
        }
        // ... repeat for asks ...
        buf.push(b'\n');

        // 3. One single I/O operation for the whole line
        let _ = w.write_all(buf);
    }
}
