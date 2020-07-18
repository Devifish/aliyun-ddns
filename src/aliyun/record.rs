use crate::aliyun::common::{request, CommonResponse, ErrorResponse};
use crate::config::Options;
use crate::error::CommonError;
use crate::ip::{self, IPOption};
use fehler::{throw, throws};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::net::IpAddr;

/// IPV4 解析类型
const RECORD_TYPE_A: &'static str = "A";
/// IPV6 解析类型
const RECORD_TYPE_AAAA: &'static str = "AAAA";

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordsResponse {
    #[serde(rename(deserialize = "RequestId"))]
    request_id: String,
    #[serde(rename(deserialize = "PageNumber"))]
    page_number: i32,
    #[serde(rename(deserialize = "TotalCount"))]
    total_count: i32,
    #[serde(rename(deserialize = "PageSize"))]
    page_size: i32,
    #[serde(rename(deserialize = "DomainRecords"))]
    pub(crate) domain_records: DomainRecords,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRecords {
    #[serde(rename(deserialize = "Record"))]
    pub(crate) record: Vec<Record>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename(deserialize = "RR"))]
    rr: String,
    #[serde(rename(deserialize = "Line"))]
    line: String,
    #[serde(rename(deserialize = "Status"))]
    status: String,
    #[serde(rename(deserialize = "Locked"))]
    locked: bool,
    #[serde(rename(deserialize = "Type"))]
    kind: String,
    #[serde(rename(deserialize = "DomainName"))]
    domain_name: String,
    #[serde(rename(deserialize = "Value"))]
    value: String,
    #[serde(rename(deserialize = "RecordId"))]
    record_id: String,
    #[serde(rename(deserialize = "TTL"))]
    ttl: i32,
    #[serde(rename(deserialize = "Weight"))]
    weight: i32,
}

/// 获取域名解析记录
#[throws(CommonError)]
pub fn list(
    opt: &Options,
    records: &HashMap<String, String>,
) -> (Vec<Record>, HashMap<String, String>) {
    let mut update_rs: Vec<Record> = Vec::new();
    let mut create_rs: HashMap<String, String> = HashMap::new();
    for (k, v) in records {
        let data = get(opt, k.to_string(), v.to_string())?;
        match data {
            CommonResponse::Ok(mut data) => {
                let ref mut r = data.domain_records.record;
                if r.len() > 0 {
                    update_rs.append(r);
                } else {
                    create_rs.insert(k.to_string(), v.to_string());
                }
            }
            CommonResponse::Err(err) => log::error!("检查解析错误{:?}", err),
        }
    }
    (update_rs, create_rs)
}

#[throws(CommonError)]
fn get(
    opt: &Options,
    record: String,
    domain: String,
) -> CommonResponse<RecordsResponse, ErrorResponse> {
    let ak_secret = opt.access_key_secret.clone().ok_or("缺少阿里云AK Secret")?;
    let ak_id = opt.access_key_id.clone().ok_or("缺少阿里云AK ID")?;

    let mut req_params = HashMap::new();
    req_params.insert("Action".to_string(), "DescribeDomainRecords".to_string());
    req_params.insert("DomainName".to_string(), domain);
    req_params.insert("KeyWord".to_string(), record);
    req_params.insert("SearchMode".to_string(), "EXACT".to_string());
    req_params.insert("PageSize".to_string(), "100".to_string());

    request::<CommonResponse<RecordsResponse, ErrorResponse>>(&ak_id, &ak_secret, req_params)?
}

/// 分离多个域名中解析记录与域名的关系
/// @Return HashMap<String, String> key: 域名， value: 解析记录
#[throws(CommonError)]
pub fn split(opt: &Options) -> HashMap<String, String> {
    let domains = opt.domains.clone();
    let mut records: HashMap<String, String> = HashMap::new();
    for domain in domains {
        let segments: Vec<&str> = domain.split(".").collect();
        let l = segments.len();
        if l <= 2 {
            throw!(format!("域名{}不合法，必须使用二级或以下域名", domain))
        }

        let record = segments[0..(l - 2)].join(".");
        let top_level_domain = segments[(l - 2)..l].join(".");
        records.insert(record, top_level_domain);
    }
    records
}

/// 更新解析记录
#[throws(CommonError)]
pub fn update_records(opt: &Options, ips: &ip::IPOption, records: &Vec<Record>) {
    let ak_secret = opt.access_key_secret.clone().ok_or("缺少阿里云AK Secret")?;
    let ak_id = opt.access_key_id.clone().ok_or("缺少阿里云AK ID")?;
    #[allow(unused_assignments)]
    let mut ipv4: Option<IpAddr> = None;
    let mut ipv6: Option<IpAddr> = None;
    match *ips {
        IPOption::IPV4(v4) => ipv4 = Some(IpAddr::V4(v4)),
        IPOption::IPAll(v4, v6) => {
            ipv4 = Some(IpAddr::V4(v4));
            ipv6 = Some(IpAddr::V6(v6));
        }
    }

    for r in records {
        let mut ip: Option<IpAddr> = None;
        if r.kind == RECORD_TYPE_AAAA {
            if None == ipv6 {
                throw!("没有获取到local IPv6地址，无法更新AAAA解析");
            }
            if r.value == ipv6.unwrap().to_string() {
                log::info!("{}类型解析值相同，略过更新", RECORD_TYPE_AAAA);
                continue;
            }
            ip = ipv6;
        } else if r.kind == RECORD_TYPE_A {
            if r.value == ipv4.unwrap().to_string() {
                log::info!("{}类型解析值相同，略过更新", RECORD_TYPE_A);
                continue;
            }
            ip = ipv4;
        }
        if let Err(e) = update_record(
            &ip,
            &ak_id,
            &ak_secret,
            &r.rr,
            &r.domain_name,
            &r.record_id,
            &r.kind,
        ) {
            throw!(e);
        } else {
            if r.status == "DISABLE" {
                log::info!("设置类型{}解析记录{}为enable状态", r.kind, r.rr,);
                if let Err(e) = enable_record(&ak_id, &ak_secret, &r.record_id) {
                    log::error!(
                        "设置类型{}解析记录{}为enable状态出错: {:?}",
                        r.kind,
                        r.rr,
                        e,
                    );
                }
            }
            log::info!(
                "更新解析记录成功! 域名: {}, 解析值:{}, ips:{:?}",
                &r.domain_name,
                &r.rr,
                ip,
            );
        }
    }
}

/// 组装更新解析记录所需的参数
#[throws(CommonError)]
fn update_record(
    ip: &Option<IpAddr>,
    ak_id: &str,
    ak_secret: &str,
    record: &str,
    domain: &str,
    record_id: &str,
    kind: &str,
) {
    let ip = ip.expect("更新解析时，传递空的IP地址");

    let mut req_params = HashMap::new();
    req_params.insert("Action".to_string(), "UpdateDomainRecord".to_string());
    req_params.insert("DomainName".to_string(), domain.to_string());
    req_params.insert("RR".to_string(), record.to_string());
    req_params.insert("RecordId".to_string(), record_id.to_string());
    req_params.insert("Value".to_string(), ip.to_string());
    req_params.insert("Type".to_string(), kind.to_string());

    log::info!("更新{}解析: {}, record_id:{}", kind, ip, record_id);
    request::<Value>(ak_id, ak_secret, req_params)?
}

/// 设置解析记录状态为enable
#[throws(CommonError)]
fn enable_record(ak_id: &str, ak_secret: &str, record_id: &str) {
    let mut req_params = HashMap::new();
    req_params.insert("Action".to_string(), "SetDomainRecordStatus".to_string());
    req_params.insert("Status".to_string(), "Enable".to_string());
    req_params.insert("RecordId".to_string(), record_id.to_string());
    request::<Value>(ak_id, ak_secret, req_params)?
}

/// 创建解析记录
#[throws(CommonError)]
pub fn create_records(opt: &Options, ips: &ip::IPOption, records: &HashMap<String, String>) {
    let ak_secret = opt.access_key_secret.clone().ok_or("缺少阿里云AK Secret")?;
    let ak_id = opt.access_key_id.clone().ok_or("缺少阿里云AK ID")?;
    for (record, domain) in records {
        log::info!(
            "开始创建解析! 域名: {}, 解析值:{}, ips:{:?}",
            domain,
            record,
            ips
        );
        if let Err(e) = create_record(ips, &ak_id, &ak_secret, record, domain) {
            throw!(e);
        } else {
            log::info!(
                "创建解析成功! 域名: {}, 解析值:{}, ips:{:?}",
                domain,
                record,
                ips
            );
        }
    }
}

/// 组装参数; 根据IPv4 IPv6类型分别创建记录
#[throws(CommonError)]
fn create_record(ips: &ip::IPOption, ak_id: &str, ak_secret: &str, record: &str, domain: &str) {
    let mut req_params = HashMap::new();
    req_params.insert("Action".to_string(), "AddDomainRecord".to_string());
    req_params.insert("DomainName".to_string(), domain.to_string());
    req_params.insert("RR".to_string(), record.to_string());

    match ips {
        IPOption::IPV4(v4) => {
            req_params.insert("Value".to_string(), v4.to_string());
            req_params.insert("Type".to_string(), RECORD_TYPE_A.to_string());
            let _r = request::<Value>(ak_id, ak_secret, req_params)?;
        }
        IPOption::IPAll(v4, v6) => {
            let mut v6_params = req_params.clone();
            v6_params.insert("Value".to_string(), v6.to_string());
            v6_params.insert("Type".to_string(), RECORD_TYPE_AAAA.to_string());
            let _r = request::<Value>(ak_id, ak_secret, v6_params)?;

            let mut v4_params = req_params;
            v4_params.insert("Value".to_string(), v4.to_string());
            v4_params.insert("Type".to_string(), RECORD_TYPE_A.to_string());
            let _r = request::<Value>(ak_id, ak_secret, v4_params)?;
        }
    }
}
