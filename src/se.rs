use crate::model::{api::Response, Channel};
use std::error::Error;

trait Serializer {
    fn to_xml(&self, ch: &Channel, resp: &Response) -> Result<String, Box<dyn Error>>;
}

pub mod itune {
    use super::Serializer;
    use crate::model::{api::Response, Channel};
    use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
    use quick_xml::{Reader, Writer};
    use std::error::Error;
    use std::io::Cursor;

    const XML_RSS: &str = r##"
<?xml version="1.0" encoding="UTF-8"?>
<rss xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd" version="2.0">
<channel>
<title>Example Podpcast</title>
<description>Description of podcast.</description>
<itunes:image href="http://www.example.com/podcast-icon.jpg" />
<language>en-us</language>
<itunes:category text="Sports">
  <itunes:category text="Wilderness"/>
</itunes:category>
<itunes:explicit>false</itunes:explicit>
<itunes:author>John Doe</itunes:author>
<link>http://www.example.com/</link>
<itunes:owner>
    <itunes:name>Owner Name</itunes:name>
    <itunes:email>me@example.com</itunes:email>
</itunes:owner>
</channel>
</rss>

    "##;

    const XML_ITEM: &str = r##"
<item>
    <title>Episode 1</title>
    <enclosure url="http://example.com/podcast1.mp3" type="audio/mpeg" length="1024"/>
    <guid>http://example.com/podcast1</guid>
    <pubDate>Thu, 21 Dec 2016 11:00:00 +0000</pubDate>
    <description>Description 1</description>
    <itunes:duration>600</itunes:duration>
    <link>http://example.com/podcast1</link>
</item> 
    "##;
    pub struct Client {}

    impl Serializer for Client {
        fn to_xml(&self, ch: &Channel, resp: &Response) -> Result<String, Box<dyn Error>> {
            let mut root_reader = Reader::from_str(XML_RSS);
            root_reader.trim_text(true);
            let mut root_writer = Writer::new(Cursor::new(Vec::new()));
            let mut buf = Vec::new();
            'xml: loop {
                match root_reader.read_event(&mut buf){
                    Ok(Event::Eof) => break 'xml,
                    Ok(e) => root_writer.write_event(e)?,
                    Err(e) => panic!("Error at position {}: {:?}", root_reader.buffer_position(), e),
                }
                buf.clear();
            }

            Ok(String::from_utf8(buf)?)
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
