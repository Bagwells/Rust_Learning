use time::{PrimitiveDateTime, Duration};

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let gigasecond = Duration::seconds(1_000_000_000);
    start + gigasecond
}