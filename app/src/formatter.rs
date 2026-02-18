use crate::quote::{Quote, QuoteOwned};
use std::io::Write;

pub struct Formatter {
    line_buf: Vec<u8>,
    itoa_buf: itoa::Buffer,
}

impl Formatter {
    pub fn new() -> Self {
        Self { line_buf: Vec::with_capacity(1024), itoa_buf: itoa::Buffer::new() }
    }

    #[inline(always)]
    pub fn format_owned(&mut self, q: &QuoteOwned) -> &[u8] {
        self.line_buf.clear();
        let b = &mut self.line_buf;
        let itoa = &mut self.itoa_buf;

        b.extend_from_slice(itoa.format(q.pkt_sec).as_bytes());
        b.push(b'.');
        let _ = write!(b, "{:06}", q.pkt_usec);
        b.push(b',');

        b.extend_from_slice(itoa.format(q.accept_time).as_bytes());
        b.push(b',');

        Self::append_trimmed(b, &q.issue_code);

        for i in (0..5).rev() {
            b.push(b',');
            let (p_arr, q_arr) = &q.bids[i];
            Self::append_trimmed(b, q_arr); // Qty
            b.push(b',');
            Self::append_trimmed(b, p_arr); // Price
        }

        b.push(b'\n');
        &self.line_buf
    }

    #[inline(always)]
    pub fn format_borrowed(&mut self, q: &Quote) -> &[u8] {
        self.line_buf.clear();
        let b = &mut self.line_buf;
        let itoa = &mut self.itoa_buf;

        b.extend_from_slice(itoa.format(q.pkt_sec).as_bytes());
        b.push(b'.');
        let _ = write!(b, "{:06}", q.pkt_usec);
        b.push(b',');
        b.extend_from_slice(itoa.format(q.accept_time).as_bytes());
        b.push(b',');
        b.extend_from_slice(q.issue_code.as_bytes());

        for i in (0..5).rev() {
            b.push(b',');
            b.extend_from_slice(q.bids[i].qty.as_bytes());
            b.push(b',');
            b.extend_from_slice(q.bids[i].price.as_bytes());
        }

        b.push(b'\n');
        &self.line_buf
    }

    #[inline(always)]
    fn append_trimmed(dest: &mut Vec<u8>, src: &[u8]) {
        let len = src.iter().position(|&x| x == b' ' || x == 0).unwrap_or(src.len());
        dest.extend_from_slice(&src[..len]);
    }
}
