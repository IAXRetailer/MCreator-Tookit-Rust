use std::collections::HashMap;

use reqwest::header;

pub fn get_text(url:&str) -> String {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36 Edg/109.0.1518.61"
            .parse()
            .unwrap(),
    );
    let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();
    let resp = client.get(url).send().unwrap();
    let text=resp.text().unwrap();
    return  text;
}

pub fn get_json(url:&str) -> HashMap<String,String> {
    let resp = reqwest::blocking::get(url).expect("load url error");
    let json=resp.json().unwrap();
    return  json;
}