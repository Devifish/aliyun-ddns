use crate::error::CommonError;
use fehler::throws;
use reqwest;
use std::net::{Ipv4Addr, Ipv6Addr};
use tokio::runtime::Runtime;

#[derive(Debug)]
pub enum IPOption {
    IPV4(Ipv4Addr),
    IPAll(Ipv4Addr, Ipv6Addr),
}

impl IPOption {
    #[throws(CommonError)]
    fn combine_ipv4(ipv4_str: String) -> IPOption {
        IPOption::IPV4(ipv4_str.parse::<Ipv4Addr>()?)
    }

    #[throws(CommonError)]
    fn combine_ipv6(self, res: Result<String, CommonError>) -> IPOption {
        if let IPOption::IPV4(v4) = self {
            match res {
                Ok(ipv6_str) => IPOption::IPAll(v4, ipv6_str.parse::<Ipv6Addr>()?),
                Err(e) => {
                    println!("request ipv6 address error: {:?}", e);
                    IPOption::IPV4(v4)
                }
            }
        } else {
            self
        }
    }
}

#[throws(CommonError)]
async fn get_ip(url: &str) -> String {
    let data = reqwest::get(url).await?.text().await?;
    let infos = data.split(",").collect::<Vec<&str>>();
    infos[1].to_string()
}

#[throws(CommonError)]
pub fn get_ips() -> IPOption {
    Runtime::new().unwrap().block_on(async {
        let ipv4 = get_ip("http://ip4.me/api/");
        let ipv6 = get_ip("http://ip6only.me/api/");
        let (rv4, rv6) = tokio::join!(ipv4, ipv6);
        IPOption::combine_ipv4(rv4?)?.combine_ipv6(rv6)
    })?
}
