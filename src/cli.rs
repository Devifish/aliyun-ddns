use clap::{clap_app, value_t, ArgMatches};
use std::env;
use std::str::FromStr;

const OPTION_AKID: &str = "AKID";
const OPTION_AKSCT: &str = "AKSCT";
const OPTION_DOMAIN: &str = "DOMAIN";
const OPTION_PERIOD: &str = "PERIOD";
const OPTION_TTL: &str = "TTL";

/// 命令行参数
#[derive(Clone, Debug)]
pub struct Options {
    pub access_key_id: Option<String>,
    pub access_key_secret: Option<String>,
    pub region_id: String,
    pub domains: Vec<String>,
    pub period: u32,
    pub ttl: u32,
    pub mode: Mode,
}

impl Options {
    fn new(mode: Mode) -> Self {
        Options {
            access_key_id: None,
            access_key_secret: None,
            region_id: String::from("cn-hangzhou"),
            domains: Vec::default(),
            period: 600,
            ttl: 600,
            mode: mode,
        }
    }

    fn build_matches<'a>() -> ArgMatches<'a> {
        let app = clap_app!((super::NAME) =>
            (version: super::VERSION)
            (author: super::AUTHORS)
            (about: "阿里云DDNS动态域名工具")
            (@arg VERBOSE: -v "设置日志等级")
            (@arg MODE: -m --mode +takes_value default_value("cli") possible_values(&["cli", "env"]) "运行模式")

            (@arg (OPTION_AKID) : -i --akid +takes_value required_if("MODE", "cli") requires[AKSCT DOMAIN] "阿里云 Access Key ID")
            (@arg (OPTION_AKSCT) : -s --aksct +takes_value requires[AKID DOMAIN] "阿里云 Access Key Secret")
            (@arg (OPTION_DOMAIN) : -d --domain +takes_value requires[AKID AKID] "需要更新的域名，如多个域名需使用 “,” 分隔")
            (@arg (OPTION_PERIOD) : -p --period +takes_value default_value("600") "域名解析更新时间，建议与TTL值一致")
            (@arg (OPTION_TTL) : -t --ttl +takes_value default_value("600") "域名解析TTL值")
        );

        app.get_matches()
    }

    /// 通过命令行参数构建
    pub fn from_args() -> Self {
        let mut options = Options::new(Mode::Cli);
        let matches = Options::build_matches();

        //从命令行参数内获取
        if let Some(var) = matches.value_of(OPTION_AKID) {
            options.access_key_id = Some(var.to_string());
        }
        if let Some(var) = matches.value_of(OPTION_AKSCT) {
            options.access_key_secret = Some(var.to_string());
        }
        if let Some(var) = matches.value_of(OPTION_DOMAIN) {
            options.domains = Options::sqlit_domain(var);
        }
        if let Some(var) = matches.value_of(OPTION_PERIOD) {
            options.period = var.parse().unwrap();
        }
        if let Some(var) = matches.value_of(OPTION_TTL) {
            options.ttl = var.parse().unwrap();
        }
        options.mode = value_t!(matches, "MODE", Mode).unwrap();
        options
    }

    /// 通过环境变量参数构建
    pub fn from_env() -> Self {
        let mut options = Options::new(Mode::Env);

        //从环境变量内获取, 如存在则覆盖原值
        if let Ok(var) = env::var(OPTION_AKID) {
            options.access_key_id = Some(var);
        }
        if let Ok(var) = env::var(OPTION_AKSCT) {
            options.access_key_secret = Some(var);
        }
        if let Ok(var) = env::var(OPTION_DOMAIN) {
            options.domains = Options::sqlit_domain(&var);
        }
        if let Ok(var) = env::var(OPTION_PERIOD) {
            options.period = var.parse().unwrap();
        }
        if let Ok(var) = env::var(OPTION_TTL) {
            options.ttl = var.parse().unwrap();
        }
        options
    }



    /// 校验必填参数
    pub fn verify(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn sqlit_domain(domains: &str) -> Vec<String> {
        if domains.contains(",") {
            domains.split(",").map(|s| s.to_string()).collect()
        } else {
            vec![String::from(domains)]
        }
    }
}

#[derive(Clone, Debug)]
pub enum Mode {
    Cli,
    Env,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cli" => Ok(Mode::Cli),
            "env" => Ok(Mode::Env),
            _ => Err("no match"),
        }
    }
}
