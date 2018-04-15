/// Utility function for converting from millisecond-precision time to .NET Ticks
/// for katsuki-backwards compatiblity


//https://msdn.microsoft.com/en-us/library/system.timespan.ticks(v=vs.110).aspx
const TICKS_PER_MS: u64 = 10000;

pub fn ms_to_ticks(ms: u64) -> u64 {
    ms * TICKS_PER_MS
}

pub fn ticks_to_ms(ms: u64) -> u64 {
    ms / TICKS_PER_MS
}