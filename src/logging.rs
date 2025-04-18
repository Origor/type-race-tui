
use std::{error::Error, fs::File, io};
use log::{info, LevelFilter};
use simplelog::{ConfigBuilder, WriteLogger};

pub fn setup_logging() -> Result<(), Box<dyn Error>> {
    if let Err(error) = std::fs::create_dir_all("/app/logs") {
        eprintln!(
            "NOTICE: logging::setup_logging() - Could not create log directory /app/logs: {}",
            error
        );
    }

    match File::create("/app/logs/app.log") {
        Ok(log_file) => {
            let config = ConfigBuilder::new()
                .set_time_format_rfc3339()
                .set_location_level(LevelFilter::Debug)
                .set_target_level(LevelFilter::Debug)
                .set_thread_level(LevelFilter::Trace)
                .set_level_padding(simplelog::LevelPadding::Right)
                .build();
            if WriteLogger::init(LevelFilter::max(), config, log_file).is_err() {
                eprintln!("ERROR: logging::setup_logging() - Failed to initialize file logger!");
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to initialize file logger",
                )));
            } else {
                info!("Logging initialized successfully.");
                return Ok(());
            }
        },
        Err(error) => {
            eprintln!(
                "ERROR: logging::setup_logging() - Failed to create log file '/app/logs/app.log': {}",
                error
            );
            return Err(Box::new(error));
        }
    }
}
