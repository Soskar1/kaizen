use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DayLog {
    activity_duration_in_seconds: u64
}

impl DayLog {
    pub fn new(activity_duration_in_seconds: u64) -> Self {
        Self {
            activity_duration_in_seconds
        }
    }

    pub fn activity_duration(&self) -> u64 {
        self.activity_duration_in_seconds
    }
}