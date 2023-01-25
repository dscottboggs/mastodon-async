use derive_deref::Deref;
use std::time::Duration;

/// How long to wait before checking an endpoint again.
#[derive(Debug, Deref, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PollingTime(Duration);

impl Default for PollingTime {
    fn default() -> Self {
        Self(Duration::from_millis(500))
    }
}

impl From<Duration> for PollingTime {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}
