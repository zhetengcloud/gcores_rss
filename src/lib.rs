pub mod model;
pub mod request;
pub mod se;
pub use service::get;

mod service {
    use crate::model::Channel;
    use crate::request::req;
    use crate::se::{itune, Serializer};
    use std::error::Error;

    pub async fn get<'a>(
        param: req::Param,
        ch_info: Channel<'a>,
    ) -> Result<String, Box<dyn Error>> {
        let fetch_client = req::Client {};
        let resp = fetch_client.fetch(param).await?;
        let serializer = itune::Client::default();
        let xml_str = serializer.to_xml(&ch_info, &resp)?;
        Ok(xml_str)
    }
}
