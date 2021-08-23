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
        .map(|(id, secret, req): (String, String, req::Request)| {
            format!("{},{}\ntitle:{}", id, secret, req.channel.title)
        });

    warp::serve(route).run(([0, 0, 0, 0], 9000)).await;
}

#[allow(dead_code)]
mod req {
    use gcores_rss::{Channel, Param};
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Request {
        #[serde(rename = "storage_param")]
        pub oss_param: S3Param,
        pub channel: Channel,
        pub param: Param,
    }

    #[derive(Deserialize)]
    pub struct S3Param {
        pub service: String,
        pub bucket: String,
        pub key: String,
        pub acl: Option<String>,
        pub content_type: Option<String>,
    }
} /* req */
