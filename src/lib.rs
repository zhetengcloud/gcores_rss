pub mod model;
pub mod request;
pub mod se;

mod service {
    use crate::model::Channel;
    use crate::request::req;
    use crate::se::{itune, Serializer};
    use std::error::Error;

    pub struct Param<'a> {
        fetch_param: req::Param,
        ch_info: Channel<'a>,
    }

    pub async fn get<'a>(param: &Param<'a>) -> Result<String, Box<dyn Error>> {
        let fetch_client = req::Client {};
        let resp = fetch_client.fetch(param.fetch_param.clone()).await?;
        let serializer = itune::Client::default();
        let xml_str = serializer.to_xml(&param.ch_info, &resp)?;
        Ok(xml_str)
    }
}
