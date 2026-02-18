use crate::quote::{Quote, QuoteOwned};
use std::collections::BinaryHeap;

pub struct ReorderBuffer {
    pub heap: BinaryHeap<QuoteOwned>,
    pub max_time_seen: u64,
}

impl ReorderBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { heap: BinaryHeap::with_capacity(capacity), max_time_seen: 0 }
    }

    #[inline(always)]
    pub fn add(&mut self, quote: Quote, seq: u64) {
        if quote.accept_time > self.max_time_seen {
            self.max_time_seen = quote.accept_time;
        }
        self.heap.push(quote.to_owned(seq));
    }
}
