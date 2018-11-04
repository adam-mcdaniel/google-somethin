use reqwest::header::USER_AGENT;
use reqwest::Client;

use select::document::Document;
use select::predicate::*;

use std::str;

#[derive(Debug)]
pub struct ValueConstructor {
    pub link: String,
    pub title: String,
}

impl Clone for ValueConstructor {
    fn clone(&self) -> Self {
        ValueConstructor {
            link: self.link.clone(),
            title: self.title.clone(),
        }
    }
}

pub struct GoogleClient;

pub trait Construct {
    fn new() -> Self;
}

impl Construct for GoogleClient {
    fn new() -> Self {
        GoogleClient
    }
}

impl GoogleClient {
    fn grab_body(&self, url: &str) -> String {
        let client = Client::new();
        let mut res = client.get(url)
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.10; rv:34.0) Gecko/20100101 Firefox/34.0")
            .send()
            .unwrap();

        res.text().unwrap()
    }

    fn grab_sections(&self, body: String) -> Vec<ValueConstructor> {
        let document = Document::from(body.as_str());

        let mut results: Vec<ValueConstructor> = Vec::new();

        for node in document.find(Attr("id", "ires").descendant(Class("bkWMgd"))) {
            for elms in node.find(Class("r").descendant(Name("a"))) {
                let mut title = elms.text();
                let link = elms.attr("href").unwrap();

                if (link.starts_with("http://") || link.starts_with("https://")) && title != "Cached" 
                {
                    title = title.replace(link, "");
                    results.push(ValueConstructor {
                        title,
                        link: link.to_string(),
                    })
                } else {
                    continue;
                }
            }
        }

        results
    }

    pub fn return_sections(
        &self,
        url: &str,
        limit: i32,
        log_to_console: bool,
    ) -> Vec<ValueConstructor> {
        let request_string = format!(
            "https://www.google.com/search?q={}&gws_rd=ssl&num={}",
            url, limit
        );

        let results = self.grab_sections(self.grab_body(request_string.as_str()));

        if log_to_console == true {
            for result in results.clone() {
                println!("Title : {} | Link : {}", result.title, result.link);
            }
        }

        results
    }
}
