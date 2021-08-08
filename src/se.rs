use crate::model::api::Response;
use crate::model::Channel;
use std::error::Error;

trait Serializer {
    fn to_xml(&self, ch: &Channel, resp: &Response) -> Result<String, Box<dyn Error>>;
}

pub mod itune {
    use super::Serializer;
    use crate::model::{api::Response, Channel};
    use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
    use quick_xml::Writer;
    use std::error::Error;
    use std::io::Cursor;

    pub struct Client {}

    impl Serializer for Client {
        fn to_xml(&self, ch: &Channel, resp: &Response) -> Result<String, Box<dyn Error>> {
            Ok("one two".to_string())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Client;
        use crate::se::Serializer;
        use crate::model::{api::Response, Channel};
        use std::error::Error;

        #[test]
        fn se_xml() -> Result<(), Box<dyn Error>> {
            let itune = Client {};
            let ch = Channel {
                title: "test podcast",
                description: "some desc",
                image: "http://www.example.com/podcast-icon.jpg",
                author: "John Doe",
                link: "http://example.com",
                owner_name: "some owner",
                owner_email: "some@eee.com",
                media_base_url: "https://example.com/media/",
                ..Default::default()
            };
            let json: String = std::fs::read_to_string("api_response.json")?;
            let response: Response = serde_json::from_str(&json)?;
            let xml_str = itune.to_xml(&ch, &response)?;
            println!("{}", xml_str);
            Ok(())
        }
    }
}
