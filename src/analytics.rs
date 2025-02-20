use std::{collections::HashMap, time::Duration};
use chrono::{DateTime, Utc};
use crate::parser::ParsedLog;

pub struct LogAnalytics {
    window_size: i64,
    error_counts: HashMap<String, usize>,
    response_times: Vec<f64>,
    user_activity: HashMap<String, Vec<DateTime<Utc>>>,
    resource_usage: HashMap<String, Vec<(DateTime<Utc>, f64)>>,
    window_start: DateTime<Utc>
}

impl LogAnalytics {
    pub fn new(window_size : i64) -> Self {
        Self { window_size, error_counts: HashMap::new(), 
            response_times: Vec::new(),
            user_activity: HashMap::new(),
            resource_usage: HashMap::new(),
            window_start: Utc::now()
        }
    }

    pub fn process_log(&mut self, log: ParsedLog) {
        // prune the data
        self.prune_old_data();



    
    }

    pub fn prune_old_data(&mut self) {

        let current_time = Utc::now();
        let window_start = current_time - Duration::from_secs(self.window_size);

        self.window_start = window_start;

        for activities in self.user_activity.values_mut() {
            activities.retain(|&timestamp| timestamp >= window_start);
        }

        for usaged in self.user_activity.values_mut() {
            usaged.retain(|&timestamp| timestamp >= window_start);
        }
    }


    pub fn get_response_time_percentiles(&self) -> Option<(f64, f64, f64)>{

        if self.response_times.is_empty() {
            return None;
        }

        // get teh percentile score for the response times
        let mut response_times = self.response_times.clone();
        response_times.sort_by(|a,b| a.partial_cmp(b).unwrap());

        let len = response_times.len();

        let percentile50 = ((len as f64) * 0.50) as usize;
        let percentile90 = ((len as f64) * 0.90) as usize;
        let percentile99 = ((len as f64) * 0.99) as usize;


        Some((
            response_times[percentile50],
            response_times[percentile90],
            response_times[percentile99]
        ))
    }

    pub fn get_active_users_count(&self) -> usize {
        self.user_activity.values()
        .filter(|activities| !activities.is_empty())
        .count()
    }

    pub fn get_resource_usage_trend(&self,resource : &str) -> Option<f64> {
        self.resource_usage.get(resource)
        .map(|usage_data| {
            if usage_data.is_empty() {
                return 0.0;
            }

            let sum = usage_data.iter().map(|(_ ,value)| value).sum::<f64>();
            sum / usage_data.len() as f64
        })
    }
}