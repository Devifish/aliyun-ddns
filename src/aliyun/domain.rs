use crate::aliyun::common::{request, CommonResponse, ErrorResponse};
use crate::config::Options;
use crate::error::CommonError;
use fehler::throws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainListResponse {
    #[serde(rename(deserialize = "RequestId"))]
    request_id: String,
    #[serde(rename(deserialize = "PageNumber"))]
    page_number: i32,
    #[serde(rename(deserialize = "TotalCount"))]
    total_count: i32,
    #[serde(rename(deserialize = "PageSize"))]
    page_size: i32,
    #[serde(rename(deserialize = "Domains"))]
    domains: DomainList,
}

#[derive(Serialize, Deserialize, Debug)]
struct DomainList {
    #[serde(rename(deserialize = "Domain"))]
    domain: Vec<Domain>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Domain {
    #[serde(rename(deserialize = "AliDomain"))]
    ali_domain: bool,
    #[serde(rename(deserialize = "ResourceGroupId"))]
    resource_group_id: String,
    #[serde(rename(deserialize = "DomainName"))]
    domain_name: String,
    #[serde(rename(deserialize = "CreateTime"))]
    create_time: String,
    #[serde(rename(deserialize = "PunyCode"))]
    puny_code: String,
    #[serde(rename(deserialize = "DnsServers"))]
    dns_servers: DnsServer,
    #[serde(rename(deserialize = "Starmark"))]
    star_mark: bool,
    #[serde(rename(deserialize = "VersionCode"))]
    version_code: String,
    #[serde(rename(deserialize = "DomainId"))]
    domain_id: String,
    #[serde(rename(deserialize = "VersionName"))]
    version_name: String,
    #[serde(rename(deserialize = "RecordCount"))]
    record_count: i32,
    #[serde(rename(deserialize = "CreateTimestamp"))]
    create_timestamp: i64,
    #[serde(rename(deserialize = "Tags"))]
    tags: Tags,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tags {
    #[serde(rename(deserialize = "Tag"))]
    tag: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DnsServer {
    #[serde(rename(deserialize = "DnsServer"))]
    dns_server: Vec<String>,
}

/// 获取所有的域名
#[throws(CommonError)]
pub fn list(opt: &Options) -> CommonResponse<DomainListResponse, ErrorResponse> {
    let ak_secret = opt.access_key_secret.clone().ok_or("缺少阿里云AK Secret")?;
    let ak_id = opt.access_key_id.clone().ok_or("缺少阿里云AK ID")?;

    let mut req_params = HashMap::new();
    req_params.insert("Action".to_string(), "DescribeDomains".to_string());
    req_params.insert("PageSize".to_string(), "100".to_string());
    // let req_method = "GET".to_string();

    // let (sign_str, params) = sign(req_method, ak_secret, ak_id, req_params)?;
    // let mut url = Url::parse(aliyun::ALIYUN_API).unwrap();
    // url.query_pairs_mut()
    //     .append_pair("Signature", sign_str.as_str());
    // for (k, v) in params.iter() {
    //     url.query_pairs_mut().append_pair(k, v);
    // }

    request::<CommonResponse<DomainListResponse, ErrorResponse>>(&ak_id, &ak_secret, req_params)?
}
