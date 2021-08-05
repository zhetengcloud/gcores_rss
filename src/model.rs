pub mod api {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        pub data: Vec<Radio>,
        pub included: Vec<inc::Media>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Radio {
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
