use std::fs::File;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, EnvFilter};

/// initialize tracing with a json formatter
/// returns a workerguard to ensure that the file is flushed on drop
#[allow(dead_code)]
pub(crate) fn init_json() -> Result<WorkerGuard, std::io::Error> {
    let file = File::create("logfile.json")?;
    let (non_blocking, guard) = tracing_appender::non_blocking(file);
    let filter = EnvFilter::default()
        .add_directive("hyper=info".parse().unwrap())
        .add_directive("reqwest=info".parse().unwrap())
        .add_directive("mastodon_async=trace".parse().unwrap());
    fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking)
        .json()
        .init();
    Ok(guard)
}

/// initialize tracing with the default formatter
/// returns a workerguard to ensure that the file is flushed on drop
pub(crate) fn init_default() -> Result<WorkerGuard, std::io::Error> {
    let file = File::create("logfile.txt")?;
    let (non_blocking, guard) = tracing_appender::non_blocking(file);
    let filter = EnvFilter::default()
        .add_directive("hyper=info".parse().unwrap())
        .add_directive("reqwest=info".parse().unwrap())
        .add_directive("mastodon_async=trace".parse().unwrap());
    fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking)
        .init();
    Ok(guard)
}
