pub fn now_time() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
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
