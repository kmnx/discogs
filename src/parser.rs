use serde::{Deserialize};
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::de::from_reader;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub realname: Option<String>,
    pub profile: Option<String>,
    pub data_quality: Option<String>,
    #[serde(default)]
    pub aliases: Option<Aliases>,
    #[serde(default)]
    pub namevariations: Option<NameVariations>,
    #[serde(default)]
    pub groups: Option<Groups>,
    #[serde(default)]
    pub urls: Option<Urls>,
    #[serde(default)]
    pub images: Option<Images>,
    #[serde(default)]
    pub members: Option<Members>,
}

