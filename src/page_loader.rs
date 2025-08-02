use std::{collections::HashMap, fs};

use chrono::{DateTime, FixedOffset};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

// use dioxus_lazy::

lazy_static! {
    pub static ref PAGE_KEYS: Vec<PageKey> = load_page_keys();
    pub static ref PAGE_DATA: HashMap<String, PageData> = load_page_data(&PAGE_KEYS);
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PageKey {
    pub title: String,
    pub short_name: String,
    pub timestamp: Time,
    pub path: String,
}

pub struct PageData {
    pub content: String,
}

#[derive(Debug, PartialEq, Clone)]
struct Time {
    value: DateTime<FixedOffset>,
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.to_rfc2822().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(Self {
            value: DateTime::parse_from_rfc2822(&value).map_err(serde::de::Error::custom)?,
        })
    }
}

fn load_page_keys() -> Vec<PageKey> {
    let path = "./pages/blog.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse JSON")
}

fn load_page_data(page_keys: &Vec<PageKey>) -> HashMap<String, PageData> {
    let mut page_data = HashMap::new();

    for page in page_keys {
        let content = fs::read_to_string(&page.path).expect("Unable to read file");

        page_data.insert(page.short_name.clone(), PageData { content });
    }

    page_data
}

// #[derive(PartialEq, Props, Clone)]
// struct PageProps {
//     content: String,
// }

// #[component]
// fn render(props: PageProps) -> Element {
//     rsx! {
//         MarkdownPage { content: props.content }
//     }
// }
