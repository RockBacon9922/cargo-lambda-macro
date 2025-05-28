use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload<T = ()> {
    pub version: String,
    pub route_key: String,
    pub raw_path: String,
    pub raw_query_string: String,
    pub headers: Headers,
    pub request_context: RequestContext,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64encoded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
}

impl<T: fmt::Debug> fmt::Display for Payload<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Payload {{ version: {}, route_key: {}, raw_path: {}, raw_query_string: {}, headers: {:?}, request_context: {:?}, is_base64encoded: {}, body: {:?} }}",
            self.version,
            self.route_key,
            self.raw_path,
            self.raw_query_string,
            self.headers,
            self.request_context,
            self.is_base64encoded,
            self.body
        )
    }
}

//implement default for Payload<T> where T: Default
impl<T: Default> Default for Payload<T> {
    fn default() -> Self {
        Payload {
            version: "2.0".to_string(),
            route_key: "".to_string(),
            raw_path: "".to_string(),
            raw_query_string: "".to_string(),
            headers: Headers::default(),
            request_context: RequestContext::default(),
            is_base64encoded: false,
            body: Some(T::default()),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
    #[serde(rename = "sec-fetch-mode")]
    pub sec_fetch_mode: String,
    #[serde(rename = "x-amzn-tls-version")]
    pub x_amzn_tls_version: String,
    #[serde(rename = "sec-fetch-site")]
    pub sec_fetch_site: String,
    #[serde(rename = "x-forwarded-proto")]
    pub x_forwarded_proto: String,
    #[serde(rename = "accept-language")]
    pub accept_language: String,
    #[serde(rename = "x-forwarded-port")]
    pub x_forwarded_port: String,
    #[serde(rename = "x-forwarded-for")]
    pub x_forwarded_for: String,
    pub accept: String,
    #[serde(rename = "x-amzn-tls-cipher-suite")]
    pub x_amzn_tls_cipher_suite: String,
    #[serde(rename = "x-amzn-trace-id")]
    pub x_amzn_trace_id: String,
    pub host: String,
    #[serde(rename = "accept-encoding")]
    pub accept_encoding: String,
    #[serde(rename = "user-agent")]
    pub user_agent: String,
    #[serde(rename = "sec-fetch-dest")]
    pub sec_fetch_dest: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    pub account_id: String,
    pub api_id: String,
    pub domain_name: String,
    pub domain_prefix: String,
    pub http: Http,
    pub request_id: String,
    pub route_key: String,
    pub stage: String,
    pub time: String,
    pub time_epoch: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub source_ip: String,
    pub user_agent: String,
}
