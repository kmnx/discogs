use serde::Deserialize;
use quick_xml::Reader;
use flate2::read::GzDecoder;
use quick_xml::de::from_reader;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Writer;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

#[derive(Debug, Deserialize)]
pub struct Aliases {
    #[serde(rename = "name")]
    pub names: Option<Vec<AliasName>>,
}

#[derive(Debug, Deserialize)]
pub struct AliasName {
    #[serde(rename = "id")]
    pub id: Option<u32>,
    #[serde(rename = "$text")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NameVariations {
    #[serde(rename = "name")]
    pub names: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Groups {}

#[derive(Debug, Deserialize)]
pub struct Urls {
    #[serde(rename = "url")]
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Images {}

#[derive(Debug, Deserialize)]
pub struct Members {
    #[serde(rename = "name")]
    pub names: Option<Vec<MemberName>>,
}

#[derive(Debug, Deserialize)]
pub struct MemberName {
    #[serde(rename = "id")]
    pub id: Option<u32>,
    #[serde(rename = "$text")]
    pub name: Option<String>,
}





fn parse_artists_xml_to_csv(input_file: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(input_file)?;
    let gz = GzDecoder::new(file);
    let mut reader = Reader::from_reader(BufReader::new(gz));

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) if e.name().as_ref() == b"artist" => {
                let mut artist_buf = Vec::new();
                let mut depth = 1;
                buf.clear();
                while depth > 0 {
                    match reader.read_event_into(&mut buf)? {
                        Event::Start(_) => {
                            depth += 1;
                            artist_buf.extend_from_slice(&buf);
                        }
                        Event::End(ref e) if e.name().as_ref() == b"artist" => {
                            depth -= 1;
                            artist_buf.extend_from_slice(&buf);
                        }
                        Event::Eof => break,
                        _ => {
                            artist_buf.extend_from_slice(&buf);
                        }
                    }
                    buf.clear();
                }
                let artist: Artist = from_reader(artist_buf.as_slice())?;
                // Use artist struct as needed
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    Ok(())
}