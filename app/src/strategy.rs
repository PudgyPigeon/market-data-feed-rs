use crate::processor::Processor;
use crate::quote::Quote;
use std::io::Write;

pub trait ProcessingStrategy {
    fn handle(&self, processor: &mut Processor, quote: Quote, seq: u64);
}

pub struct ImmediateMode;
pub struct ReorderMode;

pub enum StrategyType {
    ImmediateMode(ImmediateMode),
    ReorderMode(ReorderMode),
}

impl ProcessingStrategy for ImmediateMode {
    #[inline(always)]
    fn handle(&self, processor: &mut Processor, quote: Quote, _seq: u64) {
        let bytes = processor.formatter.format_borrowed(&quote);
        let _ = processor.writer.write_all(bytes);
    }
}

impl ProcessingStrategy for ReorderMode {
    #[inline(always)]
    fn handle(&self, processor: &mut Processor, quote: Quote, seq: u64) {
        processor.buffer.add(quote, seq);
        processor.drain_expired();
    }
}
