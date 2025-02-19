use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub struct LogAnalytics {
    window_size: i64,
    error_counts: HashMap<String, usize>,
    response_times: Vec<f64>,
    user_activity: HashMap<String, Vec<DateTime<Utc>>>,
    resource_usage: HashMap<String, Vec<DateTime<Utc>>>,
    window_start: DateTime<Utc>
}