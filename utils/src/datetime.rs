pub fn now_time() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Local::now().fixed_offset()
}

pub fn now_timestamp(checked_add: Option<i64>) -> i64 {
    use std::ops::Add;
    let timestamp = now_time().timestamp();
    if let Some(time) = checked_add {
        return timestamp.add(time);
    }
    timestamp
}

pub fn to_local_string(datetime: chrono::DateTime<chrono::FixedOffset>) -> String {
    datetime
        .with_timezone(&chrono::Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

pub fn parse_string(datetime: String) -> chrono::DateTime<chrono::FixedOffset> {
    chrono::NaiveDate::parse_from_str(datetime.as_str(), "%Y-%m-%d")
        .map(|date| {
            let time = chrono::NaiveTime::from_hms_opt(00, 00, 00).unwrap_or_default();
            let local_datetime = date.and_time(time);
            offset_from_timestamp(local_datetime.timestamp())
        })
        .unwrap_or_else(|_x| now_time())
}

pub fn offset_from_timestamp(timestamp: i64) -> chrono::DateTime<chrono::FixedOffset> {
    let utc_time = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp, 0).unwrap();
    utc_time.fixed_offset()
}
