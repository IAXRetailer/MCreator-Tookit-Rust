use std::collections::HashMap;

pub fn get_text(url:&str) -> String {
    let resp = reqwest::blocking::get(url).expect("load url error");
    let text=resp.text().unwrap();
    return  text;
}

pub fn get_json(url:&str) -> HashMap<String,String> {
    let resp = reqwest::blocking::get(url).expect("load url error");
    let json=resp.json().unwrap();
    return  json;
}