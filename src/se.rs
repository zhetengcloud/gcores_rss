use crate::model::api::Response;
use std::error::Error;

pub trait Serializer {
    fn to_xml(&self, resp: &Response) -> Result<String, Box<dyn Error>>;
}

pub(crate) mod util {
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use quick_xml::Writer;
    use std::error::Error;
    use std::io::Cursor;

    pub(crate) fn format_xml(xml: &str) -> Result<String, Box<dyn Error>> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(e) => assert!(writer.write_event(&e).is_ok()),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            }
            buf.clear();
        }

        let result = writer.into_inner().into_inner();
        Ok(String::from_utf8(result)?)
    }
}

pub mod itunes {
    use super::Serializer;
    use crate::model;
    use crate::model::api::{inc::Media, Radio, Response};
    use quick_xml::{se, Writer};
    use std::error::Error;
    use std::io::Cursor;

    const MPEG: &str = "audio/mpeg";
    //no file size in api response
    const SIZE_1024: u32 = 1024;
    const ZH_CN: &str = "zh-cn";
    const VERSION: &str = "2.0";
    const XMLNS_ITUNES: &str = "http://www.itunes.com/dtds/podcast-1.0.dtd";
    const XMLNS_CONTENT: &str = "http://purl.org/rss/1.0/modules/content/";
    // const XML_DECL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

    pub struct Client<'a> {
        pub ch: &'a model::Channel<'a>,
    }

    impl<'a> Client<'a> {
        fn resp_to_rss(&self, resp: &'a Response) -> xml::Rss {
            let items: Vec<xml::Item> = resp
                .data
                .iter()
                .zip(resp.included.iter())
                .map(|(radio, media)| self.to_item(radio, media))
                .collect();

            let owner = xml::Owner {
                name: self.ch.owner_name.to_owned(),
                email: self.ch.owner_email.to_owned(),
            };
            let image = xml::Image {
                href: self.ch.image.to_owned(),
            };
            let channel = xml::Channel {
                title: self.ch.title.to_owned(),
                description: self.ch.description.to_owned(),
                link: self.ch.link.to_owned(),
                language: ZH_CN.to_string(),
                category: xml::Category {
                    text: self.ch.category1.to_owned(),
                },
                image,
                explicit: self.ch.explicit,
                author: self.ch.author.to_owned(),
                owner,
                items,
            };

            xml::Rss {
                version: VERSION.to_string(),
                spec: XMLNS_ITUNES.to_string(),
                content: XMLNS_CONTENT.to_string(),
                channel,
            }
        }

        fn to_item(&self, radio: &Radio, media: &Media) -> xml::Item {
            let file_url = format!("{}{}", self.ch.media_base_url, media.attributes.audio);
            let encl = xml::Enclosure {
                url: file_url.clone(),
                m_type: MPEG.to_string(),
                length: SIZE_1024,
            };

            xml::Item {
                title: radio.attributes.title.clone(),
                enclosure: encl,
                guid: file_url,
                pub_date: radio.attributes.published_at.clone(),
                desc: radio.attributes.desc.clone(),
                duration: media.attributes.duration,
                link: format!("{}{}", self.ch.web_base_url, radio.id),
            }
        }
    }
    impl<'a> Serializer for Client<'a> {
        fn to_xml(&self, resp: &Response) -> Result<String, Box<dyn Error>> {
            let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
            let rss = self.resp_to_rss(resp);
            se::to_writer(writer.inner(), &rss)?;
            let xml_bytes = writer.into_inner().into_inner();
            Ok(String::from_utf8(xml_bytes)?)
        }
    }

    pub mod xml {
        use serde::Serialize;

        #[derive(Serialize, PartialEq, Debug)]
        #[serde(rename_all = "lowercase")]
        pub struct Rss {
            pub version: String,
            #[serde(rename = "xmlns:itunes")]
            pub spec: String,
            #[serde(rename = "xmlns:content")]
            pub content: String,
            pub channel: Channel,
        }

        #[derive(Serialize, PartialEq, Debug)]
        pub struct Channel {
            #[serde(rename = "$unflatten=title")]
            pub title: String,
            #[serde(rename = "$unflatten=description")]
            pub description: String,
            #[serde(rename = "$unflatten=link")]
            pub link: String,
            #[serde(rename = "$unflatten=language")]
            pub language: String,
            #[serde(rename = "itunes:category")]
            pub category: Category,
            #[serde(rename = "itunes:image")]
            pub image: Image,
            #[serde(rename = "$unflatten=explicit")]
            pub explicit: bool,
            #[serde(rename = "$unflatten=author")]
            pub author: String,
            #[serde(rename = "itunes:owner")]
            pub owner: Owner,
            pub items: Vec<Item>,
        }

        #[derive(Serialize, PartialEq, Debug)]
        pub struct Category {
            pub text: String,
        }

        #[derive(Serialize, PartialEq, Debug)]
        pub struct Image {
            pub href: String,
        }

        #[derive(Serialize, PartialEq, Debug)]
        pub struct Owner {
            #[serde(rename = "itunes:name")]
            pub name: String,
            #[serde(rename = "itunes:email")]
            pub email: String,
        }

        #[derive(Serialize, PartialEq, Debug)]
        #[serde(tag = "item")]
        pub struct Item {
            #[serde(rename = "title")]
            pub title: String,
            pub enclosure: Enclosure,
            #[serde(rename = "guid")]
            pub guid: String,
            #[serde(rename = "pubDate")]
            pub pub_date: String,
            #[serde(rename = "description")]
            pub desc: String,
            #[serde(rename = "itunes:duration")]
            pub duration: u16,
            #[serde(rename = "link")]
            pub link: String,
        }

        #[derive(Serialize, PartialEq, Debug)]
        #[serde(tag = "enclosure")]
        pub struct Enclosure {
            pub url: String,
            #[serde(rename = "type")]
            pub m_type: String,
            pub length: u32,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Client;
        use crate::model::{api::Response, Channel};
        use crate::se::util;
        use crate::se::Serializer;
        use std::error::Error;

        fn make_ch<'a>() -> Channel<'a> {
            Channel {
                title: "soeme podcast",
                author: "soem author",
                description: "some description 123",
                image: "exm.com/icon.jpg",
                language: "lang1",
                category1: "Leisure",
                category2: "Cook",
                explicit: false,
                link: "exm.com/site",
                owner_name: "john doe",
                owner_email: "john@exm.com",
                media_base_url: "exm.com/files/",
                web_base_url: "exm.com/page/",
            }
        }

        #[test]
        fn test_to_rss() -> Result<(), Box<dyn Error>> {
            let resp_bytes = std::include_bytes!("../api_response.json");
            let resp_str = String::from_utf8(resp_bytes.to_vec())?;
            let resp: Response = serde_json::from_str(&resp_str)?;
            let client = Client { ch: &make_ch() };
            let rss_str = client.to_xml(&resp)?;
            println!("{}", util::format_xml(&rss_str)?);

            Ok(())
        }
    }
}
