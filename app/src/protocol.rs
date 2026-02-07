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
    issue_code_length: 12,
    price_length: 5,
    qty_length: 7,
    accept_time_length: 8,
    level_length: 12,
    end_of_msg_val: 0xff,
};
