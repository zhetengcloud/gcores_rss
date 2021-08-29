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
        .and(warp::body::bytes())
        .map(|id, secret, data: bytes::Bytes| {
            let req: req::Request = serde_json::from_slice(&data).unwrap();
            (id, secret, req)
        })
        .untuple_one()
        .and_then(|id, secret, req: req::Request| async move {
            let req::Request {
                mut oss_param,
                param,
                channel,
            } = req;
            let xml: String = gcores_rss::get(param, channel).await.unwrap();

            oss_param.access_id = Some(id);
            oss_param.access_secret = Some(secret);

            Ok::<(req::OssParam, String), warp::reject::Rejection>((oss_param, xml))
        })
        .untuple_one()
        .and_then(|param, xml| async move {
            let resp = req::save_to_oss(param, xml);
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

    pub fn save_to_oss(param: OssParam, xml: String) -> String {
        println!("save to oss {:?}",param);
        let OssParam {
            endpoint,
            bucket,
            key,
            access_id,
            access_secret,
            ..
        } = param;
        let mut buf: Vec<u8> = Vec::new();
        let mut easy = Easy::new();
        easy.url(format!("http://{}.{}/{}", bucket, endpoint, key).as_ref())
            .unwrap();

        easy.put(true).unwrap();

        let format_date = util::get_date();
        let mut headers = List::new();
        let auth = aliyun::oss::Client {
            verb: "PUT".to_string(),
            content: vec![],
            oss_headers: [].to_vec(),
            bucket: bucket.clone(),
            content_type: "".to_string(),
            date: Some(format_date.clone()),
            key,
            key_id: access_id.expect("access id error"),
            key_secret: access_secret.expect("access secret error"),
        };

        headers
            .append(&format!("authorization: {}", auth.make_authorization()))
            .unwrap();
        headers.append(&format!("Host: {}.{}", bucket, endpoint)).unwrap();
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
        #[serde(skip)]
        pub access_id: Option<String>,
        #[serde(skip)]
        pub access_secret: Option<String>,
    }
} /* req */
