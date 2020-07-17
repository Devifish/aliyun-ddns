use clap::{clap_app, ArgMatches};

pub fn init<'a>() -> ArgMatches<'a> {
    let app = clap_app!((super::NAME) =>
        (version: super::VERSION)
        (author: super::AUTHORS)
        (about: "阿里云DDNS动态域名工具")
        (@arg VERBOSE: -v "设置日志等级")
        (@arg MODE: -m --mode +takes_value default_value("cli") possible_values(&["cli", "env"]) "运行模式")

        (@arg (super::OPTION_AKID) : -i --akid +takes_value required_if("MODE", "cli") requires[AKSCT DOMAIN] "阿里云 Access Key ID")
        (@arg (super::OPTION_AKSCT) : -s --aksct +takes_value requires[AKID DOMAIN] "阿里云 Access Key Secret")
        (@arg (super::OPTION_DOMAIN) : -d --domain +takes_value requires[AKID AKSCT] "需要更新的域名，如多个域名需使用 “,” 分隔")
        (@arg (super::OPTION_PERIOD) : -p --period +takes_value default_value("600") "域名解析更新时间，建议与TTL值一致")
        (@arg (super::OPTION_TTL) : -t --ttl +takes_value default_value("600") "域名解析TTL值")
    );

    app.get_matches()
}
