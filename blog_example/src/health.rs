use once_cell::sync::Lazy;
use serde::Serialize;
use std::time::{Duration, SystemTime};

use crate::response::ApiResponse;

static START_TIME: Lazy<SystemTime> = Lazy::new(SystemTime::now);

#[derive(Debug, Serialize)]
pub struct HealthStatus {
    status: &'static str,
    uptime_seconds: u64,
    uptime_human: String,
}

pub fn get_health_status() -> HealthStatus {
    let uptime = START_TIME.elapsed().unwrap_or(Duration::from_secs(0));
    let uptime_seconds = uptime.as_secs();

    HealthStatus {
        status: "ok",
        uptime_seconds,
        uptime_human: format_duration(uptime),
    }
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let remaining_seconds = seconds % 60;

    match (days, hours, minutes) {
        (0, 0, 0) => format!("{}s", remaining_seconds),
        (0, 0, _) => format!("{}m {}s", minutes, remaining_seconds),
        (0, _, _) => format!("{}h {}m {}s", hours, minutes, remaining_seconds),
        (_, _, _) => format!("{}d {}h {}m {}s", days, hours, minutes, remaining_seconds),
    }
}
