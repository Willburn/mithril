use clap::Parser;
use slog::{o, Drain, Level, Logger};
use slog_scope::debug;
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, path::PathBuf};

use mithril_signer::{
    Config, ProductionServiceBuilder, ServiceBuilder, SignerRunner, SignerState, StateMachine,
};

/// CLI args
#[derive(Parser)]
#[clap(name = "mithril-signer")]
#[clap(about = "An implementation of a Mithril Signer", long_about = None)]
#[command(version)]
pub struct Args {
    /// Run Mode
    #[clap(short, long, env("RUN_MODE"), default_value = "dev")]
    run_mode: String,

    /// Verbosity level
    #[clap(
        short,
        long,
        action = clap::ArgAction::Count,
        help = "Verbosity level, add more v to increase"
    )]
    verbose: u8,

    /// Configuration file location
    #[clap(
        short,
        long,
        default_value = "./config",
        help = "Directory where the configuration file is located"
    )]
    configuration_dir: PathBuf,
}

impl Args {
    fn log_level(&self) -> Level {
        match self.verbose {
            0 => Level::Warning,
            1 => Level::Info,
            2 => Level::Debug,
            _ => Level::Trace,
        }
    }
}

fn build_logger(min_level: Level) -> Logger {
    let drain = slog_bunyan::new(std::io::stdout())
        .set_pretty(false)
        .build()
        .fuse();
    let drain = slog::LevelFilter::new(drain, min_level).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    Logger::root(Arc::new(drain), o!())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load args
    let args = Args::parse();
    let _guard = slog_scope::set_global_logger(build_logger(args.log_level()));

    // Load config
    let config: Config = config::Config::builder()
        .add_source(
            config::File::with_name(&format!(
                "{}/{}.json",
                args.configuration_dir.display(),
                args.run_mode
            ))
            .required(false),
        )
        .add_source(config::Environment::default())
        .build()
        .map_err(|e| format!("configuration build error: {}", e))?
        .try_deserialize()
        .map_err(|e| format!("configuration deserialize error: {}", e))?;
    debug!("Started"; "run_mode" => &args.run_mode, "config" => format!("{:?}", config));

    let mut state_machine = StateMachine::new(
        SignerState::Unregistered(None),
        Box::new(SignerRunner::new(
            config.clone(),
            ProductionServiceBuilder::new(&config).build()?,
        )),
        Duration::from_millis(config.run_interval),
    );
    state_machine.run().await
}
