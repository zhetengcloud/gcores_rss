use crate::model::api::Response;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use simple_error::SimpleError;
use std::io::Cursor;

trait Serializer {
    fn to_xml(&self, resp: &Response) -> Result<String, SimpleError>;
}

pub struct Itune {
    version: String,
    xmlns: String,
}

impl Serializer for Itune {
    fn to_xml(&self, resp: &Response) -> Result<String, SimpleError> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        writer
            .write_event(Event::Decl(BytesDecl::new(b"1.0", None, None)))
            .expect("decl error");

        writer.write_event(Event::Eof).expect("eof error");

        let data = writer.into_inner().into_inner();
        String::from_utf8(data).map_err(|x| SimpleError::new(x.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::api::Response;
    use std::error::Error;
    use std::fs;

    #[test]
    fn se_xml() -> Result<(), Box<dyn Error>> {
        let itune = Itune {
            xmlns: "dtd1".to_string(),
            version: "2.0".to_string(),
        };
        let json: String = fs::read_to_string("api_response.json").expect("file reading error");
        let response: Response = serde_json::from_str(&json).expect("json deserialization error");
        let xml_str = itune.to_xml(&response)?;
        println!("{}", xml_str);
        Ok(())
    }
}
