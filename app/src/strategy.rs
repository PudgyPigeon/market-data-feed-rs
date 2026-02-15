use crate::processor::Processor;
use crate::quote::Quote;

pub struct ImmediateMode;
pub struct ReorderMode;

pub trait ProcessingStrategy {
    // For struct to be Generic of Type: ProcessingStrategy , it must implement this function.
    fn handle(&self, processor: &mut Processor, quote: Quote, seq: u64);
}

pub enum StrategyType {
    ImmediateMode(ImmediateMode),
    ReorderMode(ReorderMode),
}

impl ProcessingStrategy for ImmediateMode {
    #[inline(always)]
    fn handle(&self, processor: &mut Processor, quote: Quote, _sequence_counter: u64) {
        processor.print_borrowed(&quote);
    }
}

impl ProcessingStrategy for ReorderMode {
    #[inline(always)]
    fn handle(&self, processor: &mut Processor, quote: Quote, sequence_counter: u64) {
        processor.buffer_quote(quote, sequence_counter);
        processor.drain_heap();
    }
}
