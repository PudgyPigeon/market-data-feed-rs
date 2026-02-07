use crate::protocol::KOSPI_LAYOUT;
use crate::quote::Quote;

fn parse_quote<'packet>(
    packet: &pcap::Packet<'packet>,
    packet_offset: usize,
) -> Option<Quote<'packet>> {
    let payload = packet.data.get(packet_offset..)?;
    Quote::from_bytes(payload, &KOSPI_LAYOUT)
}

pub fn process_packet(packet: &pcap::Packet, packet_offset: usize) {
    let Some(quote) = parse_quote(packet, packet_offset) else { return };
    println!("{:#?}", quote);
    println!(
        "[{}] Issue: {} | Best Bid: {} | Best Ask: {} | Ask Qty: {},",
        quote.accept_time,
        quote.issue_code.trim(),
        quote.bids[0].price,
        quote.asks[0].price,
        quote.asks[0].qty,
    );
}
