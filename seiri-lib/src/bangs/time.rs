/// Utility function for converting from millisecond-precision time to .NET Ticks
/// for katsuki-backwards compatiblity


//https://msdn.microsoft.com/en-us/library/system.timespan.ticks(v=vs.110).aspx
const TICKS_PER_MS: i64 = 10000;
const NS_PER_TICK: i64 = 100;
const SEC_PER_MS: i64 = 1000;
use humantime::Duration;

pub fn ticks_to_ms(ticks: i64) -> i32 {
    (ticks / TICKS_PER_MS) as i32
}

pub fn ms_to_ticks(secs: i32) -> i64 {
    secs as i64 * TICKS_PER_MS
}

pub trait TickRepr {
    fn to_ticks(&self) -> i64;
}

impl TickRepr for Duration {
    fn to_ticks(&self) -> i64 {
        let secs = self.as_secs() as i64;
        let nanos = self.subsec_nanos() as i64;
        let ticks = secs * SEC_PER_MS * TICKS_PER_MS + (nanos / NS_PER_TICK);
        ticks
    }
}
