use crate::model::api::Response;
use crate::model::Channel;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::error::Error;
use std::io::Cursor;

trait Serializer {
    fn to_xml(&self, ch: Channel, resp: &Response) -> Result<String, Box<dyn Error>>;
}

pub struct Itune<'a> {
    version: (&'a str, &'a str),
    xmlns: (&'a str, &'a str),
    prefix: &'a str,
    text_tag: &'a str,
}

static RSS: &str = "rss";
static CHANNEL: &str = "channel";
static TITLE: &str = "title";
static DESCRIPTION: &str = "description";
static LANGUAGE: &str = "language";
static CATEGORY: &str = "language";
static EXPLICIT: &str = "explicit";
static AUTHOR: &str = "author";
static LINK: &str = "link";

impl<'a> Default for Itune<'a> {
    fn default() -> Self {
        Itune {
            xmlns: ("xmlns:itunes", "http://www.itunes.com/dtds/podcast-1.0.dtd"),
            version: ("version", "2.0"),
            prefix: "itunes:",
            text_tag: "text",
        }
    }
}

impl<'a> Serializer for Itune<'a> {
    fn to_xml(&self, ch: Channel, _resp: &Response) -> Result<String, Box<dyn Error>> {
        //ascii space 32
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), 32u8, 2);
        writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;
        //rss tag
        let mut rss_tag = BytesStart::owned(RSS.as_bytes(), RSS.len());
        rss_tag.push_attribute(self.xmlns);
        rss_tag.push_attribute(self.version);
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
        writer.write_event(Event::Text(BytesText::from_plain_str(ch.title)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(TITLE.as_bytes())))?;

        //description
        writer.write_event(Event::Start(BytesStart::owned(
            DESCRIPTION.as_bytes(),
            DESCRIPTION.len(),
        )))?;
        writer.write_event(Event::CData(BytesText::from_plain_str(ch.description)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(DESCRIPTION.as_bytes())))?;

        //language
        writer.write_event(Event::Start(BytesStart::owned(
            LANGUAGE.as_bytes(),
            LANGUAGE.len(),
        )))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(ch.language)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(LANGUAGE.as_bytes())))?;

        //category
        let cat_str = format!("{}{}", self.prefix, CATEGORY);
        let arr: &[u8] = cat_str.as_bytes();
        let arr_len = arr.len();
        let mut cat1 = BytesStart::owned(arr, arr_len);
        cat1.push_attribute((self.text_tag, ch.category1));
        writer.write_event(Event::Start(cat1))?;

        let mut cat2 = BytesStart::owned(arr, arr_len);
        cat2.push_attribute((self.text_tag, ch.category2));
        writer.write_event(Event::Empty(cat2))?;
        writer.write_event(Event::End(BytesEnd::borrowed(arr)))?;

        //explicit
        writer.write_event(Event::Start(BytesStart::owned(
            EXPLICIT.as_bytes(),
            EXPLICIT.len(),
        )))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(ch.explicit)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(EXPLICIT.as_bytes())))?;

        //author
        let aut_tag = format!("{}{}", self.prefix, AUTHOR);
        let aut_arr = aut_tag.as_bytes();
        let len_aut = aut_arr.len();
        writer.write_event(Event::Start(BytesStart::owned(aut_arr, len_aut)))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(ch.author)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(aut_arr)))?;

        //link
        let link = LINK.as_bytes();
        writer.write_event(Event::Start(BytesStart::owned(link, LINK.len())))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(ch.link)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(link)))?;

        //end
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
        let itune = Itune::default();
        let ch = Channel {
            title: "test podcast",
            description: "some desc",
            image: "http://www.example.com/podcast-icon.jpg",
            author: "John Doe",
            link: "http://example.com",
            ..Default::default()
        };
        let json: String = fs::read_to_string("api_response.json")?;
        let response: Response = serde_json::from_str(&json)?;
        let xml_str = itune.to_xml(ch, &response)?;
        println!("{}", xml_str);
        Ok(())
    }
}
