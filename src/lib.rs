mod model;
mod request;
mod se;
pub use model::Channel;
pub use request::req::Param;
pub use service::get;

mod service {
    use crate::model::Channel;
    use crate::request::req;
    use crate::se::{itune, Serializer};
    use std::error::Error;

    pub async fn get(param: req::Param, ch_info: Channel) -> Result<String, Box<dyn Error>> {
        let fetch_client = req::Client {};
        let resp = fetch_client.fetch(param)?;
        let serializer = itune::Client::default();
        let xml_str = serializer.to_xml(&ch_info, &resp)?;
        Ok(xml_str)
    }
}
