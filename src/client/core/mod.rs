use reqwest::blocking::Response;
use serde::{de::DeserializeOwned, Serialize};

pub mod config;

const BASE_URL: &str = "https://api.switch-bot.com/v1.0";

fn parse<T: DeserializeOwned>(resp: Response) -> T {
    let result = resp.json::<T>();

    match result {
        Err(why) => {
            panic!("{:#?}", why);
        }
        Ok(json) => {
            return json;
        }
    }
}

pub fn get<T: DeserializeOwned>(path: &str) -> T {
    let client = reqwest::blocking::Client::new();
    let result = client
        .get(BASE_URL.to_string() + path)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf8",
        )
        .header(reqwest::header::AUTHORIZATION, config::get_token())
        .send();

    match result {
        Err(why) => {
            panic!("{:#?}", why)
        }
        Ok(resp) => {
            return parse::<T>(resp);
        }
    }
}

pub fn post<T: Serialize>(path: &str, body: T) {
    let client = reqwest::blocking::Client::new();
    let result = client
        .post(BASE_URL.to_string() + path)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/json; charset=utf8",
        )
        .header(reqwest::header::AUTHORIZATION, config::get_token())
        .json(&body)
        .send();

    match result {
        Err(why) => {
            panic!("{:#?}", why)
        }
        Ok(_) => return,
    }
}
