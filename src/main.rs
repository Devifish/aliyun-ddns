use aliyunddns::{cli::Mode, cli::Options, logger};

fn main() {
    let mut options = Options::from_args();
    if let Mode::Docker = options.mode {
        // 当使用Docker方式运行时，通过环境变量获取参数
        options = Options::from_env();
    }

    logger::init();
    aliyunddns::start(options);
}
