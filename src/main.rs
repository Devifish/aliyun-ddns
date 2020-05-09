use aliyunddns::{config::Mode, config::Options, logger, argument};
use clap::value_t;

fn main() {
    let args = argument::init();
    let options = match value_t!(args, "MODE", Mode).unwrap() {
        Mode::Cli => Options::from_args(args),
        Mode::Env => Options::from_env()
    };

    logger::init();
    aliyunddns::start(options);
}
