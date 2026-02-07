use crate::processor::Processor;
use crate::quote::Quote;

pub trait ProcessingStrategy {
    // For struct to be Type: Strategy , it must implement this function.
    fn handle(&self, processor: &mut Processor, quote: Quote, seq: u64);
}

pub struct ImmediateMode;
pub struct ReorderMode;

impl ProcessingStrategy for ImmediateMode {
    fn handle(&self, processor: &mut Processor, quote: Quote, _sequence_counter: u64) {
        processor.print_borrowed(&quote);
    }
}

impl ProcessingStrategy for ReorderMode {
    fn handle(&self, processor: &mut Processor, quote: Quote, sequence_counter: u64) {
        processor.buffer_quote(quote, sequence_counter);
        processor.drain_heap();
    }
}
