pub mod cli;
pub mod logger;
pub mod task;

use self::{cli::Options, task::DomainUpdate};

/// aliyun-ddns
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub fn start(options: Options) {
    log::info!("version: {}", VERSION);
    log::info!("mode: {:?}", options.mode);

    let update = DomainUpdate::new(options);
    update.run();
}
