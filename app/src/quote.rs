use crate::protocol::{MAX_ID_LEN, MAX_VAL_LEN, QuoteLayout};
use libc::timeval;
use std::cmp::Ordering;
use std::str;

#[derive(Debug, Copy, Clone)]
pub struct PriceQty<'packet> {
    pub price: &'packet str,
    pub qty: &'packet str,
}

#[derive(Debug, Copy, Clone)]
pub struct Quote<'packet> {
    pub pkt_sec: i64,
    pub pkt_usec: i64,
    pub accept_time: u64, // HHMMSSuu
    pub issue_code: &'packet str,
    pub bids: [PriceQty<'packet>; 5],
    pub asks: [PriceQty<'packet>; 5],
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct QuoteOwned {
    pub pkt_sec: i64,
    pub pkt_usec: i64,
    pub accept_time: u64,
    pub sequence_counter: u64,
    pub issue_code: [u8; MAX_ID_LEN],
    pub bids: [([u8; MAX_VAL_LEN], [u8; MAX_VAL_LEN]); 5],
    pub asks: [([u8; MAX_VAL_LEN], [u8; MAX_VAL_LEN]); 5],
}

impl Ord for QuoteOwned {
    // By flipping self and other comparison order we turn BinaryMaxHeap into MinHeap
    fn cmp(&self, other: &Self) -> Ordering {
        match other.accept_time.cmp(&self.accept_time) {
            // If times are equal, check sequence
            Ordering::Equal => other.sequence_counter.cmp(&self.sequence_counter),
            // Otherwise return Ord time comparison
            ord => ord,
        }
    }
}

impl PartialOrd for QuoteOwned {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// To avoid heap allocation
fn str_to_fixed<const N: usize>(s: &str) -> [u8; N] {
    let mut arr = [0u8; N];
    let bytes = s.as_bytes();
    let len = bytes.len().min(N);
    arr[..len].copy_from_slice(&bytes[..len]);
    arr
}

impl<'packet> Quote<'packet> {
    pub fn to_owned(self, sequence_counter: u64) -> QuoteOwned {
        QuoteOwned {
            pkt_sec: self.pkt_sec,
            pkt_usec: self.pkt_usec,
            accept_time: self.accept_time,
            sequence_counter,
            issue_code: str_to_fixed::<MAX_ID_LEN>(self.issue_code),
            bids: self.bids.map(|b| {
                (str_to_fixed::<MAX_VAL_LEN>(b.price), str_to_fixed::<MAX_VAL_LEN>(b.qty))
            }),
            asks: self.asks.map(|a| {
                (str_to_fixed::<MAX_VAL_LEN>(a.price), str_to_fixed::<MAX_VAL_LEN>(a.qty))
            }),
        }
    }

    #[inline(always)]
    pub fn from_bytes(payload: &'packet [u8], layout: &QuoteLayout, ts: timeval) -> Option<Self> {
        if payload.len() <= layout.end_of_msg_offset {
            return None;
        }
        // SAFETY: Validated payload length above already so no need for bounds check (?)
        unsafe {
            let ptr = payload.as_ptr();

            if std::ptr::read(ptr as *const [u8; 5]) != *layout.header_val
                || *ptr.add(layout.end_of_msg_offset) != layout.end_of_msg_val
            {
                return None;
            }
        }

        let s = |start, len| unsafe {
            let slice = payload.get_unchecked(start..start + len);
            std::str::from_utf8_unchecked(slice)
        };

        let base_b = layout.bids_offset;
        let base_a = layout.asks_offset;
        let step = layout.level_length;
        let p_len = layout.price_length;
        let q_len = layout.qty_length;

        let bids = [
            PriceQty { price: s(base_b, p_len), qty: s(base_b + p_len, q_len) },
            PriceQty { price: s(base_b + step, p_len), qty: s(base_b + step + p_len, q_len) },
            PriceQty {
                price: s(base_b + step * 2, p_len),
                qty: s(base_b + step * 2 + p_len, q_len),
            },
            PriceQty {
                price: s(base_b + step * 3, p_len),
                qty: s(base_b + step * 3 + p_len, q_len),
            },
            PriceQty {
                price: s(base_b + step * 4, p_len),
                qty: s(base_b + step * 4 + p_len, q_len),
            },
        ];

        let asks = [
            PriceQty { price: s(base_a, p_len), qty: s(base_a + p_len, q_len) },
            PriceQty { price: s(base_a + step, p_len), qty: s(base_a + step + p_len, q_len) },
            PriceQty {
                price: s(base_a + step * 2, p_len),
                qty: s(base_a + step * 2 + p_len, q_len),
            },
            PriceQty {
                price: s(base_a + step * 3, p_len),
                qty: s(base_a + step * 3 + p_len, q_len),
            },
            PriceQty {
                price: s(base_a + step * 4, p_len),
                qty: s(base_a + step * 4 + p_len, q_len),
            },
        ];

        let time_str = s(layout.accept_time_offset, layout.accept_time_length);
        let accept_time =
            time_str.as_bytes().iter().fold(0u64, |acc, &b| acc * 10 + (b - b'0') as u64);

        Some(Quote {
            pkt_sec: ts.tv_sec,
            pkt_usec: ts.tv_usec,
            accept_time,
            issue_code: s(layout.issue_code_offset, layout.issue_code_length),
            bids,
            asks,
        })
    }
}
