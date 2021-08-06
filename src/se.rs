use crate::model::api::Response;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use std::error::Error;
use std::io::Cursor;

trait Serializer {
    fn to_xml(&self, resp: &Response) -> Result<String, Box<dyn Error>>;
}

pub struct Itune<'a> {
    version: &'a str,
    version_val: &'a str,
    xmlns: &'a str,
    xmlns_val: &'a str,
}

static RSS: &str = "rss";
static CHANNEL: &str = "channel";
static TITLE: &str = "title";
static DTD1: &str = "http://www.itunes.com/dtds/podcast-1.0.dtd";

impl<'a> Serializer for Itune<'a> {
    fn to_xml(&self, _resp: &Response) -> Result<String, Box<dyn Error>> {
        //ascii space 32
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), 32u8, 2);
        writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;
        //rss tag
        let mut rss_tag = BytesStart::owned(RSS.as_bytes(), RSS.len());
        rss_tag.push_attribute((self.xmlns, self.xmlns_val));
        rss_tag.push_attribute((self.version, self.version_val));
        writer.write_event(Event::Start(rss_tag))?;

        //channel
        writer.write_event(Event::Start(BytesStart::owned(
            CHANNEL.as_bytes(),
            CHANNEL.len(),
        )))?;

        //title
        writer.write_event(Event::Start(BytesStart::owned(
            TITLE.as_bytes(),
            TITLE.len(),
        )))?;

        writer.write_event(Event::End(BytesEnd::borrowed(TITLE.as_bytes())))?;
        writer.write_event(Event::End(BytesEnd::borrowed(CHANNEL.as_bytes())))?;
        writer.write_event(Event::End(BytesEnd::borrowed(RSS.as_bytes())))?;

        writer.write_event(Event::Eof)?;

        let data = writer.into_inner().into_inner();
        Ok(String::from_utf8(data)?)
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
            xmlns: "xmlns:itunes",
            xmlns_val: DTD1,
            version: "version",
            version_val: "2.0",
        };
        let json: String = fs::read_to_string("api_response.json")?;
        let response: Response = serde_json::from_str(&json)?;
        let xml_str = itune.to_xml(&response)?;
        println!("{}", xml_str);
        Ok(())
    }
}
