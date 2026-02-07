use crate::protocol;
use std::str;

#[derive(Debug)]
pub struct PriceQty<'packet> {
    pub price: &'packet str,
    pub qty: &'packet str,
}

#[derive(Debug)]
pub struct Quote<'packet> {
    pub issue_code: &'packet str,
    pub bids: [PriceQty<'packet>; 5],
    pub asks: [PriceQty<'packet>; 5],
    pub accept_time: &'packet str, // HHMMSSuu
}

impl<'packet> Quote<'packet> {
    #[inline(always)]
    pub fn from_bytes(payload: &'packet [u8], layout: &protocol::QuoteLayout) -> Option<Self> {
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

        Some(Quote {
            issue_code: s(layout.issue_code_offset, layout.issue_code_length),
            bids,
            asks,
            accept_time: s(layout.accept_time_offset, layout.accept_time_length),
        })
    }
}
