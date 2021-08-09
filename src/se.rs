use crate::model::{api::Response, Channel};
use std::error::Error;

trait Serializer {
    fn to_xml(&self, ch: &Channel, resp: &Response) -> Result<String, Box<dyn Error>>;
}

pub mod itune {
    use super::Serializer;
    use crate::model::{api::Response, Channel};
    use quick_xml::events::{BytesStart, BytesText, Event};
    use quick_xml::{Reader, Writer};
    use std::error::Error;
    use std::io::Cursor;

    const XML_RSS: &str = r##"
<?xml version="1.0" encoding="UTF-8"?>
<rss xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd" version="2.0">
<channel>
<title></title>
<description></description>
<itunes:image/>
<language>zh-cn</language>
<itunes:category>
</itunes:category>
<itunes:explicit>true</itunes:explicit>
<itunes:author></itunes:author>
<link></link>
<itunes:owner>
    <itunes:name></itunes:name>
    <itunes:email></itunes:email>
</itunes:owner>
</channel>
</rss>
"##;

    #[allow(dead_code)]
    const XML_ITEM: &str = r##"
<item>
    <title></title>
    <enclosure type="audio/mpeg"/>
    <guid></guid>
    <pubDate></pubDate>
    <description></description>
    <itunes:duration></itunes:duration>
    <link></link>
</item> 
    "##;
    pub struct Client {}

    const IMAGE: &[u8] = b"itunes:image";
    const CATEGORY: &[u8] = b"itunes:category";
    const TEXT: &str = "text";
    const TITLE: &[u8] = b"title";
    const DESC: &[u8] = b"description";
    const AUTHOR: &[u8] = b"itunes:author";
    const LINK: &[u8] = b"link";
    const OWNER_NAME: &[u8] = b"itunes:name";
    const OWNER_EMAIL: &[u8] = b"itunes:email";

    impl Serializer for Client {
        fn to_xml(&self, ch: &Channel, _resp: &Response) -> Result<String, Box<dyn Error>> {
            let mut root_reader = Reader::from_str(XML_RSS);
            root_reader.trim_text(true);
            let mut root_writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
            let mut buf = Vec::new();
            'xml: loop {
                let events: Vec<Event> = match root_reader.read_event(&mut buf) {
                    Ok(Event::Eof) => break 'xml,
                    Ok(Event::Start(mut bt_st)) if bt_st.name() == CATEGORY => {
                        bt_st.push_attribute((TEXT, ch.category1));
                        let mut cat2 = BytesStart::borrowed_name(CATEGORY);
                        cat2.push_attribute((TEXT, ch.category2));
                        vec![Event::Start(bt_st), Event::Empty(cat2)]
                    }
                    Ok(Event::Start(bt_st)) if bt_st.name() == DESC => {
                        vec![
                            Event::Start(bt_st),
                            Event::CData(BytesText::from_escaped_str(ch.description)),
                        ]
                    }
                    Ok(Event::Start(bt_st)) => {
                        let plain_txt = match bt_st.name() {
                            TITLE => ch.title,
                            AUTHOR => ch.author,
                            LINK => ch.link,
                            _ => "",
                        };
                        vec![
                            Event::Start(bt_st),
                            Event::Text(BytesText::from_plain_str(plain_txt)),
                        ]
                    }
                    Ok(Event::Empty(mut bt_st)) if bt_st.name() == IMAGE => {
                        bt_st.push_attribute(("href", ch.image));
                        vec![Event::Empty(bt_st)]
                    }
                    Ok(e) => vec![e],
                    Err(e) => panic!(
                        "Error at position {}: {:?}",
                        root_reader.buffer_position(),
                        e
                    ),
                };
                for ev in events {
                    root_writer.write_event(ev)?;
                }
                buf.clear();
            }

            let result: Vec<u8> = root_writer.into_inner().into_inner();
            Ok(String::from_utf8(result)?)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Client;
        use crate::model::{api::Response, Channel};
        use crate::se::Serializer;
        use std::error::Error;

        #[test]
        fn se_xml() -> Result<(), Box<dyn Error>> {
            let itune = Client {};
            let ch = Channel {
                title: "test podcast",
                description: r#"<message> Welcome to My Channel #$@!<some desc"#,
                image: "http://www.example.com/podcast-icon.jpg",
                author: "John Doe",
                link: "http://example.com",
                owner_name: "some owner",
                owner_email: "some@eee.com",
                media_base_url: "https://example.com/media/",
                category1: "Leisure",
                category2: "Video Game",
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
