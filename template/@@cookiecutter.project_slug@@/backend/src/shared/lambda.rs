use tracing::Level;

// Initializes the logger for a lambda function.
// In debug mode (locally), it logs at DEBUG level and includes timestamps and ANSI colors.
// In production mode (on AWS), it logs at INFO level and does not include timestamps or ANSI colors.
pub fn init_logger() {
    let fmt = tracing_subscriber::fmt()
        .with_max_level(if cfg!(any(debug_assertions, test)) { Level::DEBUG } else { Level::INFO })
        .with_target(false);
    #[cfg(not(any(debug_assertions, test)))]
    let fmt = fmt.without_time().with_ansi(false);
    fmt.init();
}
