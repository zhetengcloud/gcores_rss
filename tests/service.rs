#[cfg(test)]
mod tests {
    use gcores_rss::{get, Channel, Param};

    #[tokio::test]
    async fn test_get() {
        let ch_info = Channel {
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
        let fetch_param = Param {
            url: "https://www.gcores.com/gapi/v1/radios".to_owned(),
            start: 0u16,
            size: 3u16,
        };

        let xml_str = get(fetch_param, ch_info).await.expect("get xml error");
        println!("{}", xml_str);
    }
}
