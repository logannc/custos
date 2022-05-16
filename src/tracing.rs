use dotenv::dotenv;
use std::{env, path::PathBuf};
use tracing_appender;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const LOG_FILE_PREFIX: &str = "custos.log";

// TODO: should use custos::config::Configuration
fn get_log_directory() -> PathBuf {
    if let Ok(dir) = env::var("LOG_DIRECTORY") {
        dir.into()
    } else {
        println!(
            "$LOG_DIRECTORY is unset, defaulting to current directory [{}]",
            env::current_dir()
                .expect("Unable to get current working directory :/")
                .to_str()
                .unwrap()
        );
        env::current_dir().unwrap()
    }
}

pub fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    dotenv().ok();
    let file_appender = tracing_appender::rolling::daily(get_log_directory(), LOG_FILE_PREFIX);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(non_blocking))
        // this should use RUST_LOG, test later
        // .with(EnvFilter::from_default_env())
        .with(EnvFilter::new("info,bot=trace,custos=trace"))
        .init();
    _guard
}
