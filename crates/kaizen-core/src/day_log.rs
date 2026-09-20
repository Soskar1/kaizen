use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DayLog {
    activity_duration_in_seconds: u64
}

impl DayLog {
    pub fn new(activity_duration: Duration) -> Self {
        Self {
            activity_duration_in_seconds: activity_duration.as_secs()
        }
    }

    pub fn activity_duration(&self) -> Duration {
        Duration::from_secs(self.activity_duration_in_seconds)
    }
}