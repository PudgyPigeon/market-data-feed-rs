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
    #[inline(always)]
    fn parse_8_digits_swar(s: &[u8]) -> u64 {
        let val = unsafe { std::ptr::read_unaligned(s.as_ptr() as *const u64) };

        let masked = val.wrapping_sub(0x3030303030303030);

        let mul1 = (masked & 0x00FF00FF00FF00FF).wrapping_mul(0x000A0001000A0001) >> 8;
        let mul2 = (mul1 & 0x0000FFFF0000FFFF).wrapping_mul(0x0064000100640001) >> 16;

        // Directly return the expression to satisfy clippy::let-and-return
        (mul2 & 0x00000000FFFFFFFF).wrapping_mul(0x2710000100000000) >> 32
    }

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
        // Validation: Ensure we can at least check the header and end-of-message byte
        if payload.len() <= layout.end_of_msg_offset {
            return None;
        }

        unsafe {
            let ptr = payload.as_ptr();
            let header = std::ptr::read_unaligned(ptr as *const [u8; 5]);

            if header != *layout.header_val {
                #[cfg(debug_assertions)]
                {
                    static mut UNKNOWN_COUNT: u64 = 0;
                    UNKNOWN_COUNT += 1;
                    if UNKNOWN_COUNT <= 5 {
                        eprintln!(
                            "DEBUG: Skipping header: {:?}",
                            std::str::from_utf8_unchecked(&header)
                        );
                    }
                }
                return None;
            }

            if *ptr.add(layout.end_of_msg_offset) != layout.end_of_msg_val {
                return None;
            }
        }

        // Zero-copy string helper
        let s = |start, len| unsafe {
            let slice = payload.get_unchecked(start..start + len);
            std::str::from_utf8_unchecked(slice)
        };

        let step = layout.level_length;
        let p_len = layout.price_length;
        let q_len = layout.qty_length;

        macro_rules! parse_level {
            ($base_offset:expr, $idx:expr) => {
                PriceQty {
                    price: s($base_offset + (step * $idx), p_len),
                    qty: s($base_offset + (step * $idx) + p_len, q_len),
                }
            };
        }

        let base_b = layout.bids_offset;
        let bids = [
            parse_level!(base_b, 0),
            parse_level!(base_b, 1),
            parse_level!(base_b, 2),
            parse_level!(base_b, 3),
            parse_level!(base_b, 4),
        ];

        let base_a = layout.asks_offset;
        let asks = [
            parse_level!(base_a, 0),
            parse_level!(base_a, 1),
            parse_level!(base_a, 2),
            parse_level!(base_a, 3),
            parse_level!(base_a, 4),
        ];

        // --- SWAR Time Parsing ---
        // We read exactly 8 bytes for the SWAR math.
        // Ensure your layout.accept_time_length is actually 8!
        let accept_time = unsafe {
            let time_slice =
                payload.get_unchecked(layout.accept_time_offset..layout.accept_time_offset + 8);
            Self::parse_8_digits_swar(time_slice)
        };

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
