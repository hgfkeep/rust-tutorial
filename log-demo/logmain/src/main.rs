mod logger;
use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use logger::SimpleLogger;

const LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

fn main() -> Result<(), SetLoggerError> {
    init()?;
    println!("Hello, world!");

    error!("Hello error log");
    warn!("Hello warn log");
    info!("Hello info log");
    debug!("Hello debug log");
    trace!("Hello trace log");
    Ok(())
}
