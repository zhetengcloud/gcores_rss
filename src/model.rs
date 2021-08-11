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

#[derive(Default, serde::Deserialize)]
pub struct Channel {
    pub title: String,
    pub author: String,
    pub description: String,
    pub image: String,
    pub language: String,
    pub category1: String,
    pub category2: String,
    pub link: String,
    pub owner_name: String,
    pub owner_email: String,
    pub media_base_url: String,
    pub web_base_url: String,
    pub explicit: String,
}
