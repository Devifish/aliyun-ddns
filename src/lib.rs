pub mod argument;
pub mod config;
pub mod logger;
pub mod task;

use self::{config::Options, task::DomainUpdate};

/// aliyun-ddns
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// option
pub const OPTION_AKID: &str = "AKID";
pub const OPTION_AKSCT: &str = "AKSCT";
pub const OPTION_DOMAIN: &str = "DOMAIN";
pub const OPTION_PERIOD: &str = "PERIOD";
pub const OPTION_TTL: &str = "TTL";

pub fn start(options: Options) {
    log::info!("version: {}", VERSION);
    log::info!("mode: {:?}", options.mode);

    let update = DomainUpdate::new(options);
    update.run();
}
