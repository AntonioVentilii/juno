// We archive statuses in memory up to 30 days (1h * 24 * 30).
pub const RETAIN_ARCHIVE_STATUSES_NS: u64 = 3_600_000_000_000 * 24 * 30;

// We run the monitoring one per hour.
pub const MONITORING_INTERVAL_SECONDS: u64 = 60 * 60;
