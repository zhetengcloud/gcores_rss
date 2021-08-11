use crate::model::api::Response;
use crate::model::Channel;
use std::error::Error;

trait Serializer {
    fn to_xml(&self, ch: &Channel, resp: &Response) -> Result<String, Box<dyn Error>>;
}

pub mod itune {
    use crate::model::{
        api::{inc, Radio, Response},
        Channel,
    };
    use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
    use quick_xml::Writer;
    use std::error::Error;
    use std::io::Cursor;

    pub struct Client<'a> {
        version: (&'a str, &'a str),
        xmlns: (&'a str, &'a str),
        prefix: &'a str,
        xml_version: &'a str,
        xml_encode: &'a str,
    }

    const RSS: &str = "rss";
    const CHANNEL: &str = "channel";
    const TITLE: &str = "title";
    const DESCRIPTION: &str = "description";
    const LANGUAGE: &str = "language";
    const CATEGORY: &str = "category";
    const EXPLICIT: &str = "explicit";
    const AUTHOR: &str = "author";
    const LINK: &str = "link";
    const OWNER: &str = "owner";
    const NAME: &str = "name";
    const EMAIL: &str = "email";
    const ITEM: &str = "item";
    const CLOSURE: &str = "closure";
    const URL: &str = "url";
    const DURATION: &str = "duration";
    const GUID: &str = "guid";
    const MPEG: (&str, &str) = ("type", "audio/mpeg");
    const PUBDATE: &str = "pubDate";
    const TEXT: &str = "text";

    impl<'a> Default for Client<'a> {
        fn default() -> Self {
            Client {
                xmlns: ("xmlns:itunes", "http://www.itunes.com/dtds/podcast-1.0.dtd"),
                version: ("version", "2.0"),
                prefix: "itunes:",
                xml_version: "1.0",
                xml_encode: "UTF8",
            }
        }
    }

    impl<'a> Client<'a> {
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
            let description = DESCRIPTION.as_bytes();
            let pub_date = PUBDATE.as_bytes();

            let link = LINK.as_bytes();
            let web_link = format!("{}{}", ch.web_base_url, radio.id);

            vec![
                Event::Start(BytesStart::borrowed(item, ITEM.len())),
                Event::Start(BytesStart::borrowed(title, title.len())),
                Event::Text(BytesText::from_plain_str(&radio.attributes.title)),
                Event::End(BytesEnd::borrowed(title)),
                Event::Start(BytesStart::borrowed(guid, guid.len())),
                Event::Text(BytesText::from_escaped_str(audio_url)),
                Event::End(BytesEnd::borrowed(guid)),
                Event::Start(BytesStart::borrowed(description, description.len())),
                Event::CData(BytesText::from_escaped_str(radio.attributes.desc.as_str())),
                Event::End(BytesEnd::borrowed(description)),
                Event::Empty(closure_ele),
                Event::Start(BytesStart::borrowed(pub_date, pub_date.len())),
                Event::Text(BytesText::from_escaped_str(
                    radio.attributes.published_at.as_str(),
                )),
                Event::End(BytesEnd::borrowed(pub_date)),
                Event::Start(BytesStart::owned(link, LINK.len())),
                Event::Text(BytesText::from_escaped_str(web_link)),
                Event::End(BytesEnd::borrowed(link)),
                Event::End(BytesEnd::borrowed(item)),
            ]
        }
    }

    impl<'a> super::Serializer for Client<'a> {
        fn to_xml(&self, ch: &Channel, resp: &Response) -> Result<String, Box<dyn Error>> {
            //ascii space 32
            let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), 32u8, 2);
            writer.write_event(Event::Decl(BytesDecl::new(
                self.xml_version.as_bytes(),
                Some(self.xml_encode.as_bytes()),
                None,
            )))?;
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
            cat1.push_attribute((TEXT, ch.category1));
            writer.write_event(Event::Start(cat1))?;

            let mut cat2 = BytesStart::owned(arr, arr_len);
            cat2.push_attribute((TEXT, ch.category2));
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
            resp.data
                .iter()
                .zip(resp.included.iter())
                .flat_map(|(radio, media)| self.to_item(radio, media, ch))
                .try_for_each(|ev| writer.write_event(ev))?;

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
        use super::Client;
        use crate::model::{api::Response, Channel};
        use crate::se::Serializer;
        use std::error::Error;
        use std::fs;

        #[test]
        fn se_xml() -> Result<(), Box<dyn Error>> {
            let itune = Client::default();
            let ch = Channel {
                title: "test podcast",
                description: "some desc",
                image: "http://www.example.com/podcast-icon.jpg",
                author: "John Doe",
                link: "http://example.com",
                owner_name: "some owner",
                owner_email: "some@eee.com",
                media_base_url: "https://example.com/media/",
                explicit: "true",
                language: "test language",
                category1: "Travel",
                category2: "cook",
                web_base_url: "http::/exm.com/pages/",
                ..Default::default()
            };
            let json: String = fs::read_to_string("api_response.json")?;
            let response: Response = serde_json::from_str(&json)?;
            let xml_str = itune.to_xml(&ch, &response)?;
            println!("{}", xml_str);
            Ok(())
        }
    }
}
