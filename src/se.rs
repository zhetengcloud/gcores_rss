use crate::model::api::{inc, Radio, Response};
use crate::model::Channel;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::error::Error;
use std::io::Cursor;

trait Serializer {
    fn to_xml(&self, ch: &Channel, resp: &Response) -> Result<String, Box<dyn Error>>;
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
static CATEGORY: &str = "category";
static EXPLICIT: &str = "explicit";
static AUTHOR: &str = "author";
static LINK: &str = "link";
static OWNER: &str = "owner";
static NAME: &str = "name";
static EMAIL: &str = "email";
static ITEM: &str = "item";
static CLOSURE: &str = "closure";
static URL: &str = "url";
static DURATION: &str = "duration";
static GUID: &str = "guid";
static MPEG: (&str, &str) = ("type", "audio/mpeg");

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

impl<'a> Itune<'a> {
    fn to_item(&self, radio: &'a Radio, media: &'a inc::Media, ch: &Channel) -> Vec<Event> {
        let item = ITEM.as_bytes();
        let title = TITLE.as_bytes();
        let audio_url: String = format!("{}{}", ch.media_base_url, media.attributes.audio);

        let closure = CLOSURE.as_bytes();
        let mut closure_ele = BytesStart::borrowed(closure, closure.len());
        closure_ele.push_attribute(MPEG);
        closure_ele.push_attribute((URL, audio_url.as_str()));
        closure_ele.push_attribute((DURATION, media.attributes.duration.to_string().as_str()));

        let guid = GUID.as_bytes();

        vec![
            Event::Start(BytesStart::borrowed(item, ITEM.len())),
            Event::Start(BytesStart::borrowed(title, title.len())),
            Event::Text(BytesText::from_plain_str(&radio.attributes.title)),
            Event::End(BytesEnd::borrowed(title)),
            Event::Start(BytesStart::borrowed(guid, guid.len())),
            Event::Text(BytesText::from_escaped_str(audio_url)),
            Event::End(BytesEnd::borrowed(guid)),
            Event::Empty(closure_ele),
            Event::End(BytesEnd::borrowed(item)),
        ]
    }
}

impl<'a> Serializer for Itune<'a> {
    fn to_xml(&self, ch: &Channel, resp: &Response) -> Result<String, Box<dyn Error>> {
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

        //owner
        let owner = format!("{}{}", self.prefix, OWNER);
        let name = format!("{}{}", self.prefix, NAME);
        let email = format!("{}{}", self.prefix, EMAIL);
        writer.write_event(Event::Start(BytesStart::owned(
            owner.as_bytes(),
            owner.len(),
        )))?;
        writer.write_event(Event::Start(BytesStart::owned(name.as_bytes(), name.len())))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(ch.owner_name)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(name.as_bytes())))?;
        writer.write_event(Event::Start(BytesStart::owned(
            email.as_bytes(),
            email.len(),
        )))?;
        writer.write_event(Event::Text(BytesText::from_plain_str(ch.owner_email)))?;
        writer.write_event(Event::End(BytesEnd::borrowed(email.as_bytes())))?;
        writer.write_event(Event::End(BytesEnd::borrowed(owner.as_bytes())))?;

        //item
        let events = resp
            .data
            .iter()
            .zip(resp.included.iter())
            .flat_map(|(radio, media)| self.to_item(radio, media, ch))
            .collect::<Vec<Event>>();
        for ev in events {
            writer.write_event(ev)?
        }

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
            owner_name: "some owner",
            owner_email: "some@eee.com",
            media_base_url: "https://example.com/media/",
            ..Default::default()
        };
        let json: String = fs::read_to_string("api_response.json")?;
        let response: Response = serde_json::from_str(&json)?;
        let xml_str = itune.to_xml(&ch, &response)?;
        println!("{}", xml_str);
        Ok(())
    }
}
