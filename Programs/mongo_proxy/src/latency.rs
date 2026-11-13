use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedRequest {
    pub label: String,
    pub request_bytes: usize,
    pub latency: Duration,
}

struct PendingRequest {
    label: String,
    request_bytes: usize,
    sent_at: Instant,
}

#[derive(Default)]
pub struct RequestTracker {
    pending: Mutex<HashMap<i32, PendingRequest>>,
}

impl RequestTracker {
    pub fn start(&self, request_id: i32, label: String, request_bytes: usize) {
        let request = PendingRequest {
            label,
            request_bytes,
            sent_at: Instant::now(),
        };
        self.lock().insert(request_id, request);
    }

    pub fn finish(&self, response_to: i32) -> Option<CompletedRequest> {
        let request = self.lock().remove(&response_to)?;
        Some(CompletedRequest {
            label: request.label,
            request_bytes: request.request_bytes,
            latency: request.sent_at.elapsed(),
        })
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<i32, PendingRequest>> {
        self.pending.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_reply_to_request() {
        let tracker = RequestTracker::default();
        tracker.start(7, "find test.users".to_owned(), 120);

        let completed = tracker.finish(7).unwrap();

        assert_eq!(completed.label, "find test.users");
        assert_eq!(completed.request_bytes, 120);
    }

    #[test]
    fn each_request_completes_only_once() {
        let tracker = RequestTracker::default();
        tracker.start(7, "find".to_owned(), 1);

        assert!(tracker.finish(7).is_some());
        assert!(tracker.finish(7).is_none());
    }

    #[test]
    fn unknown_reply_is_ignored() {
        assert!(RequestTracker::default().finish(99).is_none());
    }
}