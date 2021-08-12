use gcores_rss::{get, Channel, Param};
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use rusoto_core::{ByteStream, Region};
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use serde::{Deserialize, Serialize};
use simple_error::SimpleError;
use simple_logger::SimpleLogger;
use std::error::Error as SError;

#[derive(Deserialize)]
struct Request {
    bucket: String,
    key: String,
    channel: Channel,
    param: Param,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    lambda_runtime::run(handler_fn(fetch_save)).await?;
    Ok(())
}

pub(crate) async fn fetch_save(event: Request, ctx: Context) -> Result<Response, SimpleError> {
    let Request {
        bucket,
        key,
        param,
        channel,
    } = event;
    get(param, channel)
        .await
        .iter()
        .try_for_each(|xml| save_to_s3(bucket.clone().to_string(), key.clone(), xml.to_string()))
        .map(|_| Response {
            msg: "xml saved".to_owned(),
            req_id: ctx.request_id,
        })
        .map_err(|e| SimpleError::new(e.to_string()))
}

fn save_to_s3(bucket: String, key: String, val: String) -> Result<(), Box<dyn SError>> {
    S3Client::new(Region::UsEast1)
        .put_object(PutObjectRequest {
            acl: Some("public_read".to_string()),
            body: Some(ByteStream::from(val.into_bytes())),
            bucket,
            key,
            ..Default::default()
        })
        .sync()?;
    Ok(())
}
