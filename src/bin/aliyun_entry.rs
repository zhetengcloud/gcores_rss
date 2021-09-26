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

mod req {
    use gcores_rss::{Channel, Param};
    use serde::Deserialize;
    use sloppy_auth::{aliyun, util};
    //use std::io::Read;

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
            acl,
            service: _,
            content_type,
        } = param;
        let acl1 = acl.unwrap_or("public-read".to_string());
        let content_type1 = content_type.unwrap_or("application/xml".to_string());
        let req_url = format!("http://{}.{}/{}", bucket.clone(), endpoint.clone(), key);

        let STS { id, secret, token } = sts;

        let format_date = util::get_date();

        let secret_header = ("x-oss-security-token".to_string(), token);

        let acl_header = ("x-oss-object-acl".to_string(), acl1);

        let auth = aliyun::oss::Client {
            verb: "PUT".to_string(),
            oss_headers: vec![secret_header.clone(), /*acl_header.clone()*/],
            bucket: bucket.clone(),
            date: Some(format_date.clone()),
            key,
            key_id: id,
            key_secret: secret,
        };

        ureq::put(&req_url)
            .set("authorization", auth.make_authorization().as_str())
            .set("Host", &format!("{}.{}", bucket, endpoint))
            .set("Content-Type", content_type1.as_str())
            .set(&secret_header.0, &secret_header.1)
            //.set(&acl_header.0, &acl_header.1)
            .set("date", &format_date.clone())
            .send_bytes(xml.as_bytes())
            .expect("oss send xml failed")
            .into_string()
            .expect("oss response to_string failed")
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
    }
} /* req */
