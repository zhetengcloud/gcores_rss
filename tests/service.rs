#[cfg(test)]
mod tests {
    use gcores_rss::{get, Channel, Param};

    #[tokio::test]
    async fn test_get() {
        let ch_info = Channel {
            title: "test podcast".to_string(),
            description: "some desc".to_string(),
            image: "http://www.example.com/podcast-icon.jpg".to_string(),
            author: "John Doe".to_string(),
            link: "http://example.com".to_string(),
            owner_name: "some owner".to_string(),
            owner_email: "some@eee.com".to_string(),
            media_base_url: "https://example.com/media/".to_string(),
            explicit: "true".to_string(),
            language: "test language".to_string(),
            category1: "Travel".to_string(),
            category2: "cook".to_string(),
            web_base_url: "http::/exm.com/pages/".to_string(),
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
