use gcores_rss::{get, Channel, Param};
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_error::SimpleError;
use simple_logger::SimpleLogger;

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
    let xml_str: String = get(param, channel).await?;
    log::info!("{}", xml_str);
    Ok(Response {
        req_id: ctx.request_id,
        msg: xml_str,
    })
}
