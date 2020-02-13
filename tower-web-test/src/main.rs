extern crate jieba_rs;
#[macro_use]
extern crate tower_web;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::iter::FromIterator;
use std::collections::HashSet;

use jieba_rs::Jieba;
use tower_web::ServiceBuilder;

#[derive(Debug)]
struct ChineseTokenizer {
    inner: Jieba,
}

impl ChineseTokenizer {
    pub fn new() -> ChineseTokenizer {
        ChineseTokenizer { inner: Jieba::new() }
    }

    pub fn cut(&self, test: &String) -> Vec<String> {
        let words = self.inner.cut(&test, true)
            .into_iter()
            .map(|w| w.to_owned())
            .collect::<HashSet<String>>();
        let mut words = Vec::from_iter(words.into_iter());

        words.sort();
        words
    }
}

#[derive(Debug, Extract)]
struct TokenizeRequest {
    text: String
}

#[derive(Debug, Response)]
#[web(status = "200")]
struct TokenizeResponse {
    words: Vec<String>,
}

impl_web! {
    impl ChineseTokenizer {
        #[post("/tokenize")]
        #[content_type("application/json")]
        fn tokenize(&self, body: TokenizeRequest) -> Result<TokenizeResponse,()> {
            Ok(TokenizeResponse {
                words: self.cut(&body.text),
            })
        }
    }
}
               
fn main() {
    println!("hello");
    env_logger::init();
    let addr = "0.0.0.0:8081".parse().expect("invalide address");
    info!("listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(ChineseTokenizer::new())
        .run(&addr)
        .unwrap();
}
