use chrono::{DateTime, TimeZone};

pub trait ToIso8601 {
  fn to_iso8601(&self) -> String;
}

impl <Tz: TimeZone> ToIso8601 for DateTime<Tz> {
  fn to_iso8601(&self) -> String {
    self.naive_utc().format("%Y-%m-%dT%H:%M:%SZ").to_string()
  }
}