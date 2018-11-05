use reqwest::header::USER_AGENT;
use reqwest::Client;

use select::document::Document;
use select::predicate::*;

use std::str;

#[derive(Debug, Clone)]
pub struct Section {
    pub link: String,
    pub title: String,
}

fn grab_body(url: &str) -> String {
    Client::new()
        .get(url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.10; rv:34.0) Gecko/20100101 Firefox/34.0",
        )
        .send()
        .unwrap()
        .text()
        .unwrap()
}

fn grab_section(title: String, link: String) -> Section {
    Section { title, link }
}

fn log_to_console(sections: Vec<Section>) {
    for data in sections {
        println!("Link : {}\nTitle : {}", data.link, data.title);
        println!("---------------------------------------------")
    }
}

pub fn return_results(query: &str, log: bool, limit: u32) -> Vec<Section> {
    let request_string = format!(
        "https://www.google.com/search?q={}&gws_rd=ssl&num={}",
        query, limit
    );

    let body = grab_body(request_string.as_str());

    let document = Document::from(body.as_str());

    let mut sections: Vec<Section> = Vec::new();

    for node in document.find(
        Attr("id", "ires")
            .descendant(Class("bkWMgd"))
            .descendant(Class("r"))
            .descendant(Name("a")),
    ) {
        let link = node.attr("href").unwrap();
        for new_node in node.find(Class("LC20lb")) {
            sections.push(grab_section(new_node.text(), link.to_string()))
        }
    }

    if log == true {
        log_to_console(sections.clone())
    }

    sections
}
