use tracing_subscriber::EnvFilter;

pub fn init_logging(config_log_level: &str) {
    let log_dir = log_directory();
    let env_level = std::env::var("COSH_LOG").ok();

    let effective_level = env_level
        .as_deref()
        .or_else(|| std::env::var("RUST_LOG").ok().as_deref().map(|_| ""))
        .unwrap_or(config_log_level);

    let filter = if let Ok(rust_log) = std::env::var("RUST_LOG") {
        EnvFilter::try_new(&rust_log).unwrap_or_else(|_| EnvFilter::new("warn"))
    } else {
        EnvFilter::try_new(effective_level).unwrap_or_else(|_| EnvFilter::new("warn"))
    };

    if let Some(dir) = &log_dir {
        let _ = std::fs::create_dir_all(dir);
        let file_appender = tracing_appender::rolling::daily(dir, "cosh-shell.log");
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(file_appender)
            .with_ansi(false)
            .with_target(true)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .with_target(true)
            .init();
    }
}

fn log_directory() -> Option<std::path::PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(|h| std::path::PathBuf::from(h).join(".copilot-shell/logs"))
}
