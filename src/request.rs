// example of request
// https://www.gcores.com/gapi/v1/radios?page[limit]=3&filter[list-all]=0&page[offset]=12&sort=-published-at&fields[radios]=title,desc,thumb,published-at&include=media

mod url {

    pub fn concat_url(url: String, start: u16, size: u16) -> Option<String> {
        let p_at: &str = "published-at";
        let fields_bs: &str = "title,desc,thumb";
        let val_sort = format!("-{}", p_at);
        let val_fields = format!("{},{}", fields_bs, p_at);

        let size_str = &(size.to_string());
        let start_str = &(start.to_string());

        let params: Vec<(&str, &str)> = vec![
            ("page[limit]", size_str),
            ("page[offset]", start_str),
            //filter audio books
            ("filter[list-all]", "0"),
            ("sort", &val_sort),
            ("fields[radios]", &val_fields),
            ("include", "media"),
        ];

        params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .reduce(|a, b| format!("{}&{}", a, b))
            .map(|s| format!("{}?{}", url, s))
    }

    #[cfg(test)]
    mod tests {
        use super::concat_url;
        use simple_error::SimpleError;

        #[test]
        fn concat() -> Result<(), SimpleError> {
            let url1 = "https://www.gcores.com/gapi/v1/radios";
            let start = 5u16;
            let size = 3u16;
            let url2 =
                concat_url(url1.to_string(), start, size).ok_or(SimpleError::new("url error"))?;
            let expected = format!("{}?{}", url1, "page[limit]=3&page[offset]=5&sort=-published-at&fields[radios]=title,desc,thumb,published-at&include=media");
            assert_eq!(url2, expected);
            Ok(())
        }
    }
}

pub mod req {
    use super::url::concat_url;
    use crate::model::api;
    use simple_error::SimpleError;
    use std::error::Error;

    pub struct Client {}

    #[derive(Debug, Clone)]
    pub struct Param {
        url: String,
        start: u16,
        size: u16,
    }

    impl Client {
        pub async fn fetch(&self, param: Param) -> Result<api::Response, Box<dyn Error>> {
            let url1 = concat_url(param.url.clone(), param.start, param.size)
                .ok_or_else(|| SimpleError::new("url error"))?;

            let resp = reqwest::get(url1).await?.json::<api::Response>().await?;

            Ok(resp)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::error::Error;

        #[tokio::test]
        async fn get_json() -> Result<(), Box<dyn Error>> {
            let url1 = "https://www.gcores.com/gapi/v1/radios";
            let param = Param {
                url: url1.to_owned(),
                start: 3u16,
                size: 4u16,
            };
            let cl = Client {};
            let resp = cl.fetch(param).await?;
            for radio in resp.data {
                println!("{}", radio.attributes.title);
                println!("{}", radio.attributes.published_at);
            }
            for med in resp.included {
                println!("{}", med.attributes.audio);
                println!("duration: {}", med.attributes.duration);
            }
            Ok(())
        }
    }
}
