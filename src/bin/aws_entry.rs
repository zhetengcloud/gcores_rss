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
    s3_param: S3Param,
    channel: Channel,
    param: Param,
}

#[derive(Deserialize)]
struct S3Param {
    bucket: String,
    key: String,
    acl: Option<String>,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
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

fn to_simple(e: Box<dyn SError>) -> SimpleError {
    SimpleError::new(e.to_string())
}

pub(crate) async fn fetch_save(event: Request, ctx: Context) -> Result<Response, SimpleError> {
    let Request {
        s3_param,
        param,
        channel,
    } = event;
    let xml: String = get(param, channel).await.map_err(to_simple)?;
    save_to_s3(s3_param, xml.to_string()).map_err(to_simple)?;
    Ok(Response {
        req_id: ctx.request_id,
    })
}

fn save_to_s3(param: S3Param, val: String) -> Result<(), Box<dyn SError>> {
    let S3Param { acl, bucket, key } = param;
    S3Client::new(Region::UsEast1)
        .put_object(PutObjectRequest {
            acl,
            body: Some(ByteStream::from(val.into_bytes())),
            bucket,
            key,
            ..Default::default()
        })
        .sync()?;
    Ok(())
}
