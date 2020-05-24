use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct Config {
    pub authors: Option<Vec<String>>,
    pub build: Build,
    pub website: Website,
}

#[derive(Deserialize, Serialize)]
pub struct Metadata {
    pub authors: Option<Vec<Author>>,
}

#[derive(Deserialize, Serialize)]
pub struct Website {
    include: Option<Vec<String>>,
    lang: Option<String>,

    pub description: String,
    pub license: Option<String>,
    pub metadata: Option<Metadata>,
    pub theme: Option<String>,
    pub title: String,
    pub url: String,
}

impl Website {
    pub fn get_include(&self) -> Vec<String> {
        self.include
            .clone()
            .unwrap_or_else(|| vec!["blog/**/*".to_string()])
    }

    pub fn get_lang(&self) -> String {
        self.lang.clone().unwrap_or_else(|| "en".to_string())
    }

    pub fn to_json(&self) -> Value {
        json!(self)
    }
}

#[derive(Deserialize)]
pub struct Build {
    target_dir: Option<String>,
}

const DST_DIR: &str = "dst";

impl Build {
    pub fn get_target_dir(&self) -> String {
        self.target_dir
            .clone()
            .unwrap_or_else(|| DST_DIR.to_string())
    }
}

#[derive(Deserialize, Serialize)]
pub struct Author {
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub email: Option<String>,
    pub name: String,
    pub nick: Option<String>,
}

impl Author {
    pub fn to_json(&self) -> Value {
        json!(self)
    }
}
