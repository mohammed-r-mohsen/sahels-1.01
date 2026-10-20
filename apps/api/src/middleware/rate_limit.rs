use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone, Debug)]
pub struct RateLimiter {
    max_requests: usize,
    window: Duration,
    state: Arc<Mutex<HashMap<String, VecDeque<Instant>>>>,
}

impl RateLimiter {
    pub fn new(max_requests: usize, burst: usize) -> Self {
        Self {
            max_requests: max_requests.max(1),
            window: Duration::from_secs(60),
            state: Arc::new(Mutex::new(HashMap::new())),
        }
        .with_burst(burst)
    }

    pub fn with_burst(mut self, burst: usize) -> Self {
        self.max_requests = self.max_requests.min(burst.max(1));
        self
    }

    pub fn allow(&self, key: &str) -> bool {
        let now = Instant::now();
        let mut guard = self.state.lock().unwrap();
        let bucket = guard.entry(key.to_string()).or_default();

        while bucket.front().is_some_and(|ts| now.duration_since(*ts) > self.window) {
            bucket.pop_front();
        }

        if bucket.len() >= self.max_requests {
            return false;
        }

        bucket.push_back(now);
        true
    }
}
