#[inline]
fn to_centiseconds(time: u64) -> u64 {
    let hh = time / 1_00_00_00;
    let mm = (time / 1_00_00) % 100;
    let ss = (time / 1_00) % 100;
    let uu = time % 100;
    ((hh * 3600) + (mm * 60) + ss) * 100 + uu
}

#[inline]
pub fn window_has_passed(accept_time: u64, max_time_seen: u64) -> bool {
    let accept_time_centi = to_centiseconds(accept_time);
    let max_time_centi = to_centiseconds(max_time_seen);
    max_time_centi.saturating_sub(accept_time_centi) >= 300 // 3 seconds
}
