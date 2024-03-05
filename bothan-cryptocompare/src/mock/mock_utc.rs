use std::cell::Cell;

use chrono::{DateTime, NaiveDateTime, Utc};

thread_local! {
    static TIMESTAMP_MILLIS: Cell<u64> = const { Cell::new(0) };
}

pub fn now() -> DateTime<Utc> {
    DateTime::from_naive_utc_and_offset(
        TIMESTAMP_MILLIS.with(|timestamp| {
            let secs = (timestamp.get() / 1000) as i64;
            let nano_secs = ((timestamp.get() % 1000) * 1_000_000) as u32;
            NaiveDateTime::from_timestamp_opt(secs, nano_secs).expect("a valid timestamp set")
        }),
        chrono::Utc,
    )
}

pub fn set_timestamp_millis(millis: u64) {
    TIMESTAMP_MILLIS.with(|ts| ts.set(millis));
}
