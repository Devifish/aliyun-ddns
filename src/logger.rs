use simplelog::*;

/// init logger
pub fn init() {
    let term_logger = TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed);
    let logger: Box<dyn SharedLogger> = match term_logger {
        Some(var) => var,
        None => SimpleLogger::new(LevelFilter::Warn, Config::default()),
    };

    CombinedLogger::init(vec![logger]).expect("No interactive terminal");
}
