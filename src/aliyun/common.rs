use crate::error::CommonError;
use chrono::{SecondsFormat, Utc};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use fehler::throws;
use nanoid::nanoid;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS, NON_ALPHANUMERIC};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::collections::BTreeMap;
use tokio::runtime::Runtime;
use url::Url;

/// 阿里云API URL
const ALIYUN_API: &'static str = "https://alidns.aliyuncs.com/";

/// 公共参数
/// https://help.aliyun.com/document_detail/29745.html?spm=a2c4g.11186623.6.626.248a7ebbnmN79G
#[derive(Serialize, Deserialize, Debug)]
struct CommonParams {
    #[serde(rename(serialize = "Format"))]
    format: String,
    #[serde(rename(serialize = "Version"))]
    version: String,
    #[serde(rename(serialize = "AccessKeyId"))]
    access_key_id: String,
    #[serde(rename(serialize = "Signature"))]
    signature: String,
    #[serde(rename(serialize = "SignatureMethod"))]
    signature_method: String,
    #[serde(rename(serialize = "Timestamp"))]
    timestamp: String,
    #[serde(rename(serialize = "SignatureVersion"))]
    signature_version: String,
    #[serde(rename(serialize = "SignatureNonce"))]
    signature_nonce: String,
}

impl Default for CommonParams {
    fn default() -> Self {
        let mut item = CommonParams {
            format: "JSON".to_string(),
            version: "2015-01-09".to_string(),
            access_key_id: "".to_string(),
            signature: "".to_string(),
            signature_method: "HMAC-SHA1".to_string(),
            timestamp: "".to_string(),
            signature_version: "1.0".to_string(),
            signature_nonce: "".to_string(),
        };
        item.signature_nonce();
        item.timestamp();
        item
    }
}

impl CommonParams {
    fn signature_nonce(&mut self) {
        self.signature_nonce = nanoid!();
    }

    fn timestamp(&mut self) {
        self.timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    }

    pub fn access_key_id(&mut self, ak_id: String) {
        self.access_key_id = ak_id
    }
}

pub const SETS: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');
pub const SETS2: &AsciiSet = &CONTROLS.add(b'=');

/// 执行阿里云请求
#[throws(CommonError)]
pub fn request<T>(ak_id: &str, ak_secret: &str, params: HashMap<String, String>) -> T
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let req_method = "GET".to_string();
    let (sign_str, params) = sign(req_method, ak_secret.to_string(), ak_id.to_string(), params)?;

    let mut url = Url::parse(ALIYUN_API).unwrap();
    url.query_pairs_mut()
        .append_pair("Signature", sign_str.as_str());
    for (k, v) in params.iter() {
        url.query_pairs_mut().append_pair(k, v);
    }

    do_get::<T>(url)?
}

// pub const SETS2: &AsciiSet = &SETS.remove(b'*');
/// 签名，参考阿里云官方文档:
/// https://help.aliyun.com/document_detail/29747.html?spm=a2c4g.11186623.2.14.2ea52b73jEErrO
#[throws(CommonError)]
pub fn sign(
    request_method: String,
    ak_secret: String,
    ak_id: String,
    request_params: HashMap<String, String>,
) -> (String, BTreeMap<String, String>) {
    let mut common_params = CommonParams::default();
    common_params.access_key_id(ak_id);
    // 将公共参数转换成HashMap
    let json_value = serde_json::to_value(&common_params)?;
    let hm: HashMap<String, String> = serde_json::from_value(json_value)?;

    // 使用BTreeMap对参数进行字典排序
    let mut params: BTreeMap<String, String> = request_params.into_iter().collect();
    for (k, v) in hm.iter() {
        if k != "Signature" {
            params.insert(k.clone(), v.clone());
        }
    }

    // let mut encode = url::form_urlencoded::Serializer::new(String::new());
    let mut str_sgin = String::new();
    let mut index = 0;
    for (k, v) in params.iter() {
        if index == 0 {
            str_sgin = format!(
                "{}={}",
                utf8_percent_encode(k, SETS).to_string(),
                utf8_percent_encode(v, SETS).to_string()
            );
        } else {
            str_sgin = format!(
                "{}&{}={}",
                str_sgin,
                utf8_percent_encode(k, SETS).to_string(),
                utf8_percent_encode(v, SETS).to_string()
            );
        }
        index += 1;
    }
    // let mut str_sgin = encode.finish();

    str_sgin = utf8_percent_encode(&str_sgin, SETS).to_string();
    str_sgin = str_sgin.replace("%2E", ".");
    str_sgin = str_sgin.replace("%2D", "-");
    str_sgin = str_sgin.replace("%20", "+");
    // str_sgin = str_sgin.replace("%2A", "*");
    str_sgin = str_sgin.replace("~", "%7E");

    let str_params = format!("{}&%2F&{}", request_method, str_sgin);
    let sign = hmac(format!("{}&", ak_secret), str_params);
    (sign, params)
}

fn hmac(key: String, data: String) -> String {
    let mut mac = Hmac::new(Sha1::new(), key.as_bytes());
    mac.input(data.as_bytes());
    let result = mac.result();
    let code = result.code();
    // println!("{:02x?}", code);
    format!("{}", base64::encode(code))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CommonResponse<T, E> {
    Err(E),
    Ok(T),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename(deserialize = "RequestId"))]
    request_id: String,
    #[serde(rename(deserialize = "Recommend"))]
    recommend: String,
    #[serde(rename(deserialize = "Code"))]
    code: String,
    #[serde(rename(deserialize = "Message"))]
    message: String,
    #[serde(rename(deserialize = "HostId"))]
    host_id: String,
}

#[throws(CommonError)]
async fn do_request<T>(url: Url) -> T
where
    T: DeserializeOwned + std::fmt::Debug,
{
    reqwest::get(url).await?.json::<T>().await?
}

#[throws(CommonError)]
pub fn do_get<T>(url: Url) -> T
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let resp: Result<T, CommonError> = Runtime::new().unwrap().block_on(async {
        let data: T = do_request::<T>(url).await?;
        Ok(data)
    });
    resp?
}
