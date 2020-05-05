use clap::{clap_app, value_t, ArgMatches};
use std::env;
use std::str::FromStr;

const OPTION_AKID: &str = "AKID";
const OPTION_AKSCT: &str = "AKSCT";
const OPTION_DOMAIN: &str = "DOMAIN";
const OPTION_PERIOD: &str = "PERIOD";

/// 命令行参数
#[derive(Clone, Debug)]
pub struct Options {
    pub access_key_id: Option<String>,
    pub access_key_secret: Option<String>,
    pub region_id: String,
    pub period: u32,
    pub domains: Vec<String>,
    pub mode: Mode,
}

impl Options {
    pub fn new() -> Self {
        Options {
            access_key_id: None,
            access_key_secret: None,
            region_id: String::from("cn-hangzhou"),
            period: 600,
            domains: Vec::default(),
            mode: Mode::Cli,
        }
    }

    /// 通过命令行参数构建
    pub fn from_args() -> Self {
        let mut options = Options::new();
        let matches = Options::build_matches();

        options.mode = value_t!(matches, "MODE", Mode).unwrap();
        options
    }

    /// 通过环境变量参数构建
    pub fn from_env() -> Self {
        let mut options = Options::new();

        //从环境变量内获取参数, 如存在则覆盖原值
        if let Ok(var) = env::var(OPTION_AKID) {
            options.access_key_id = Some(var);
        }
        if let Ok(var) = env::var(OPTION_AKSCT) {
            options.access_key_secret = Some(var);
        }
        if let Ok(var) = env::var(OPTION_DOMAIN) {
            options.access_key_id = Some(var);
        }
        if let Ok(var) = env::var(OPTION_PERIOD) {
            options.period = var.parse().unwrap();
        }
        options
    }
    
    /// 校验必填参数
    pub fn verify(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        Ok(())
    }

    fn build_matches<'a>() -> ArgMatches<'a> {
        let app = clap_app!((super::NAME) =>
            (version: super::VERSION)
            (author: super::AUTHORS)
            (about: "阿里云DDNS动态域名工具")
            (@arg VERBOSE: -v "设置日志等级")
            (@arg MODE: -m --mode +takes_value default_value("cli") possible_values(&["cli", "docker"]) "运行模式")

            (@arg (OPTION_AKID) : -i --akid +takes_value required_if("MODE", "cli") requires[AKSCT DOMAIN] "阿里云 Access Key ID")
            (@arg (OPTION_AKSCT) : -s --aksct +takes_value requires[AKID DOMAIN] "阿里云 Access Key Secret")
            (@arg (OPTION_DOMAIN) : -d --domain +takes_value requires[AKID AKID] "需要更新的域名，如多个域名需使用 “,” 分隔")
        );

        app.get_matches()
    }
}

#[derive(Clone, Debug)]
pub enum Mode {
    Cli,
    Docker,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cli" => Ok(Mode::Cli),
            "docker" => Ok(Mode::Docker),
            _ => Err("no match"),
        }
    }
}
