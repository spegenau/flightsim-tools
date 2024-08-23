use chrono::{DateTime, Timelike};
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct Times {
    pub sched_off: String,
    pub sched_on: String,
    pub sched_in: String,
    pub sched_out: String,
}

impl Times {
    pub fn get_offblock_time(&self) -> String {
        Times::format_datetime(&self.sched_off)
    }
    pub fn get_onblock_time(&self) -> String {
        Times::format_datetime(&self.sched_on)
    }
    pub fn get_out_time(&self) -> String {
        Times::format_datetime(&self.sched_out)
    }
    pub fn get_in_time(&self) -> String {
        Times::format_datetime(&self.sched_in)
    }

    fn format_datetime(literal: &str) -> String {
        let seconds: i64 = literal.parse().unwrap_or(0);

        if seconds == 0 {
            return String::new();
        }

        let time = DateTime::from_timestamp(seconds, 0).unwrap();
        format!("{:02}:{:02}", time.hour(), time.minute(),)
    }
}
