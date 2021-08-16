use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let cl = req::Client { tag: b'{' };

        cl.handle_connection(stream);
    }
}

mod req {
    use gcores_rss::{get, Channel, Param};
    //use std::error::Error;
    use serde::Deserialize;
    use std::io::prelude::*;
    use std::net::TcpStream;

    pub(crate) struct Client {
        pub tag: u8,
    }

    impl Client {
        fn get_body(&self, buffer: &[u8]) -> Vec<u8> {
            buffer
                .iter()
                .skip_while(|&&x| x != self.tag)
                .map(|&x| x)
                .collect::<Vec<u8>>()
        }

        pub fn handle_connection(&self, mut stream: TcpStream) {
            let mut buffer = [0; 1024 * 4];
            stream.read(&mut buffer).unwrap();
            let body_slice = self.get_body(&buffer);

            let body: String = match serde_json::from_slice::<Request>(&body_slice){
                Ok(request)=> {
                    let xml: String = get();
                }
                Err(e)=> e.to_string()
            };

            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",body);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    #[derive(Deserialize)]
    struct Request {
        #[serde(rename="storage_param")]
        oss_param: S3Param,
        channel: Channel,
        param: Param,
    }

    #[derive(Deserialize)]
    struct S3Param {
        service: String,
        bucket: String,
        key: String,
        acl: Option<String>,
        content_type: Option<String>,
    }
} /* req */
