// This is system/compuer level setup for memory usage
pub const MAX_ID_LEN: usize = 20; // Sacrifice memory for speed
pub const MAX_VAL_LEN: usize = 12; // Price/qty probably not above 12 digits

// Below is definition of packet reading/parsing
pub struct QuoteLayout {
    pub header_val: &'static [u8; 5],
    pub issue_code_offset: usize,
    pub bids_offset: usize,
    pub asks_offset: usize,
    pub accept_time_offset: usize,
    pub end_of_msg_offset: usize,
    pub issue_code_length: usize,
    pub price_length: usize,
    pub qty_length: usize,
    pub accept_time_length: usize,
    pub level_length: usize,
    pub end_of_msg_val: u8,
}

pub const KOSPI_LAYOUT: QuoteLayout = QuoteLayout {
    header_val: b"B6034",
    issue_code_offset: 5,
    bids_offset: 29,
    asks_offset: 121,
    accept_time_offset: 206,
    end_of_msg_offset: 214,
    issue_code_length: 12, // Must be <= MAX_ID_LEN
    price_length: 5,       // Must be <= MAX_VAL_LEN
    qty_length: 7,         // Must be <= MAX_VAL_LEN
    accept_time_length: 8,
    level_length: 12,
    end_of_msg_val: 0xff,
};

// Should not run, just for compiler to check that there is a guard
// This block ensures our internal buffers are always
// large enough for the defined KOSPI layout.
// This will cause a compile time error if the condition is false.
// TODO: Also add unit test? Maybe redundant but local Nix CI can help
macro_rules! validate_layout {
    ($layout:expr) => {
        const _: () = {
            assert!($layout.issue_code_length <= MAX_ID_LEN, "Issue code too long");
            assert!($layout.price_length <= MAX_VAL_LEN, "Price too long");
            assert!($layout.qty_length <= MAX_VAL_LEN, "Qty too long");
        };
    };
}
validate_layout!(KOSPI_LAYOUT);