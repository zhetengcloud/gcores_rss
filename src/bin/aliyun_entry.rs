use log::LevelFilter;
use simple_logger::SimpleLogger;
use warp::Filter;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    // POST /invoke
    let route = warp::path!("invoke")
        .and(warp::header::<String>("x-fc-access-key-id"))
        .and(warp::header::<String>("x-fc-access-key-secret"))
        .and(warp::header::<String>("x-fc-security-token"))
        .and(warp::body::bytes())
        .map(|id, secret, token, data: bytes::Bytes| {
            let req: req::Request = serde_json::from_slice(&data).unwrap();
            (id, secret, token, req)
        })
        .untuple_one()
        .and_then(|id, secret, token, req: req::Request| async move {
            let req::Request {
                oss_param,
                param,
                channel,
            } = req;
            let xml: String = gcores_rss::get(param, channel).await.unwrap();

            let sts = req::STS { id, secret, token };

            Ok::<(req::OssParam, req::STS, String), warp::reject::Rejection>((oss_param, sts, xml))
        })
        .untuple_one()
        .and_then(|param, sts, xml| async move {
            let resp = req::save_to_oss(param, sts, xml);
            Ok::<String, warp::reject::Rejection>(resp)
        });

    warp::serve(route).run(([0, 0, 0, 0], 9000)).await;
}

#[allow(dead_code)]
mod req {
    use curl::easy::{Easy, List};
    use gcores_rss::{Channel, Param};
    use serde::Deserialize;
    use sloppy_auth::{aliyun, util};
    use std::io::Read;

    pub struct STS {
        pub id: String,
        pub secret: String,
        pub token: String,
    }

    pub fn save_to_oss(param: OssParam, sts: STS, xml: String) -> String {
        let OssParam {
            endpoint,
            bucket,
            key,
            ..
        } = param;

        let STS { id, secret, token } = sts;

        let mut buf: Vec<u8> = Vec::new();
        let mut easy = Easy::new();
        easy.url(format!("http://{}.{}/{}", bucket, endpoint, key).as_ref())
            .unwrap();

        easy.put(true).unwrap();

        let format_date = util::get_date();
        let mut headers = List::new();

        let secret_header = ("x-oss-security-token".to_string(), token);

        let auth = aliyun::oss::Client {
            verb: "PUT".to_string(),
            oss_headers: vec![secret_header.clone()],
            bucket: bucket.clone(),
            date: Some(format_date.clone()),
            key,
            key_id: id,
            key_secret: secret,
        };

        headers
            .append(&format!("authorization: {}", auth.make_authorization()))
            .unwrap();
        headers
            .append(&format!("Host: {}.{}", bucket, endpoint))
            .unwrap();
        headers
            .append(&format!("{}:{}", secret_header.0, secret_header.1))
            .unwrap();
        headers
            .append(&format!("date: {}", format_date.clone()))
            .unwrap();

        easy.http_headers(headers).unwrap();

        {
            let mut data = xml.as_bytes();
            let mut transfer = easy.transfer();
            transfer
                .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
                .unwrap();
            transfer
                .write_function(|data| {
                    buf.extend_from_slice(data);
                    Ok(data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }
        String::from_utf8(buf).unwrap()
    }

    #[derive(Deserialize)]
    pub struct Request {
        #[serde(rename = "storage_param")]
        pub oss_param: OssParam,
        pub channel: Channel,
        pub param: Param,
    }

    #[derive(Deserialize, Debug)]
    pub struct OssParam {
        pub service: String,
        pub bucket: String,
        pub key: String,
        pub acl: Option<String>,
        pub content_type: Option<String>,
        pub endpoint: String,
        pub access_id: Option<String>,
        pub access_secret: Option<String>,
    }
} /* req */
