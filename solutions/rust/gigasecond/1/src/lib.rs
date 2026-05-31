use time::{PrimitiveDateTime, Duration, Date, Time, Month};

pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let gigasecond = Duration::seconds(1_000_000_000);
    start + gigasecond
}

fn main() {
    let date = PrimitiveDateTime::new(
        Date::from_calendar_date(2015, Month::April, 25).unwrap(),
        Time::from_hms(22, 0, 0).unwrap()
    );
    let giga_day:PrimitiveDateTime = after(date);
    println!("{giga_day}");
     
}