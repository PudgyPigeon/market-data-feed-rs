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
    pub fn from_bytes(
        payload: &'packet [u8],
        quote_layout: &protocol::QuoteLayout,
    ) -> Option<Self> {
        if payload.len() <= quote_layout.end_of_msg_offset
            || &payload[0..5] != quote_layout.header_val
            || payload[quote_layout.end_of_msg_offset] != quote_layout.end_of_msg_val
        {
            return None;
        }

        let s = |start, len| str::from_utf8(&payload[start..start + len]).unwrap_or("");

        let parse_levels = |base_offset: usize| {
            let mut levels = [const { PriceQty { price: "", qty: "" } }; 5];

            for (i, level) in levels.iter_mut().enumerate() {
                let offset = base_offset + (i * quote_layout.level_length);
                *level = PriceQty {
                    price: s(offset, quote_layout.price_length),
                    qty: s(offset + quote_layout.price_length, quote_layout.qty_length),
                };
            }
            levels
        };

        Some(Quote {
            issue_code: s(quote_layout.issue_code_offset, quote_layout.issue_code_length),
            bids: parse_levels(quote_layout.bids_offset),
            asks: parse_levels(quote_layout.asks_offset),
            accept_time: s(quote_layout.accept_time_offset, quote_layout.accept_time_length),
        })
    }
}
