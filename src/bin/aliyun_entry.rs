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
            req::save_to_oss(param, xml).await;
            Ok::<String, warp::reject::Rejection>("saved to oss".to_string())
        });

    warp::serve(route).run(([0, 0, 0, 0], 9000)).await;
}

#[allow(dead_code)]
mod req {
    use gcores_rss::{Channel, Param};
    use serde::Deserialize;

    pub async fn save_to_oss(param: OssParam, xml: String) {
        unimplemented!()
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
        #[serde(skip)]
        pub access_id: Option<String>,
        #[serde(skip)]
        pub access_secret: Option<String>,
    }
} /* req */
