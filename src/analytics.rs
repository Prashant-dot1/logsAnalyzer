use std::{collections::HashMap, time::Duration};
use chrono::{DateTime, Utc};
use crate::parser::{Level, ParsedLog};

#[derive(Debug)]
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

        if let Some(Level::Error) = log.level {
            let error_type = log.metadata.get("error_type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();

            *self.error_counts.entry(error_type).or_insert(0) += 1;
        }


        // processing user activity
        if let Some(user_id) = log.metadata.get("userid").and_then(|v| v.as_str()) {

            self.user_activity
                .entry(user_id.to_string()).or_default()
                .push(log.timestamp.unwrap_or_else(|| Utc::now()));
        }


        // processing resouce usage
        if let Some(cpu_usage) = log.metadata.get("cpu_usage").and_then(|v| v.as_str()) {

            self.resource_usage
            .entry(cpu_usage.to_string())
            .or_default()
            .push((log.timestamp.unwrap_or_else(|| Utc::now()) , cpu_usage.parse::<f64>().unwrap()));
        }


    }

    pub fn prune_old_data(&mut self) {

        let current_time = Utc::now();
        let window_start = current_time - Duration::from_secs(self.window_size as u64);

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


#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use std::sync::{Arc, Mutex};

    fn resource_usage(i : i32) -> String {
        match i {
            0..5 => (i*10).to_string(),
            _ => ((i-5)*20).to_string()
        }
    }


    #[tokio::test]
    async fn test_prune_data() {

        let log_analytics = Arc::new(Mutex::new(LogAnalytics::new(100)));
        let mut parsed_logs = Vec::new();

        for i in 0..10 {
            let parsed_log = ParsedLog {
                timestamp : Some(Utc::now()),
                level : Some(Level::Error),
                message: format!("message {}", i),
                metadata: json!({
                    "userid" : format!("userid{}", i),
                    "cpu_usage" : resource_usage(i),
                    "error_type" : "Something is really wrong"
                }),
                ..Default::default()
            };

            parsed_logs.push(parsed_log);
        }


        let mut handlers = Vec::new();
        for log in parsed_logs.into_iter() {

            let la_clone = log_analytics.clone();

            let handler = tokio::spawn(async move {
                if let Ok(mut la) = la_clone.lock() {
                    la.process_log(log);
                }
            });

            handlers.push(handler);
        }

        for handler in handlers {
            if handler.await.is_err() {
                panic!("there is an error")
            }
        }


        // print log analytics
        println!("log analytics: {:#?}", log_analytics.clone().lock().unwrap());

        // get percentile count
        if let Ok(la) = log_analytics.clone().lock() {
            let active_user_count = la.get_active_users_count();
            println!("active user count: {}", active_user_count);

            // get resouce trend for 40.0 , need to get this test case right

            if let Some(trend) =  la.get_resource_usage_trend("40") {
                println!("resource trend for 40 is: {}",trend);
            }
        }


    }
}