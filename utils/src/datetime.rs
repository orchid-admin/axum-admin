use std::ops::Add;

pub fn now_time() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
}
pub fn timestamp(checked_add: Option<i64>) -> i64 {
    let timestamp = now_time().timestamp();
    if let Some(time) = checked_add {
        return timestamp.add(time);
    }
    timestamp
}

pub fn to_local_string(datetime: chrono::DateTime<chrono::FixedOffset>) -> String {
    datetime
        .with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap())
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

pub fn parse_string(datetime: String) -> chrono::DateTime<chrono::FixedOffset> {
    let time = chrono::NaiveTime::from_hms_opt(00, 00, 00).unwrap();

    let now_datetime = now_time();
    match chrono::NaiveDate::parse_from_str(datetime.as_str(), "%Y-%m-%d") {
        Ok(date) => {
            let local_datetime = chrono::NaiveDateTime::new(date, time);
            let tz_offset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
            chrono::TimeZone::from_local_datetime(&tz_offset, &local_datetime).unwrap()
        }
        Err(_) => now_datetime,
    }
}

pub fn offset_from_timestamp(timestamp: i64) -> chrono::DateTime<chrono::FixedOffset> {
    let naive = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();

    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive, chrono::Utc)
        .with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
}
pub fn timestamp_nanos(checked_add: Option<i64>) -> i64 {
    let nanos = now_time().timestamp_nanos_opt().unwrap_or_default();
    if let Some(time) = checked_add {
        return nanos.add(time);
    }
    nanos
}

pub fn timestamp_nanos_string(checked_add: Option<i64>) -> String {
    timestamp_nanos(checked_add).to_string()
}
