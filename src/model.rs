pub mod api {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        pub data: Vec<Radio>,
        pub included: Vec<inc::Media>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Radio {
        //link: https://www.gcores.com/radios/{id}
        pub id: String,
        pub attributes: Attribute,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "kebab-case"))]
    pub struct Attribute {
        pub title: String,
        pub desc: String,
        pub thumb: String,
        pub published_at: String,
    }

    pub mod inc {
        use serde::Deserialize;
        #[derive(Deserialize, Debug)]
        pub struct Media {
            pub id: String,
            pub attributes: Attribute,
        }

        #[derive(Deserialize, Debug)]
        pub struct Attribute {
            pub audio: String,
            pub duration: u16,
        }
    }
}

#[derive(Default)]
pub struct Channel<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub description: &'a str,
    pub image: &'a str,
    pub language: &'a str,
    pub category1: &'a str,
    pub category2: &'a str,
    pub explicit: &'a str,
    pub link: &'a str,
    pub owner_name: &'a str,
    pub owner_email: &'a str,
    pub media_base_url: &'a str,
}