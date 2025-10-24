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





pub fn parse_artists_xml_to_csv(input_file: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting artist XML parsing...");
    let mut count = 0;
    let file = File::open(input_file)?;
    let gz = GzDecoder::new(file);
    let mut reader = Reader::from_reader(BufReader::new(gz));

    let mut buf = Vec::new();

    loop {
        let event = reader.read_event_into(&mut buf)?;
        match &event {
            Event::Start(e) => {
                let name = e.name();
                let tag = String::from_utf8_lossy(name.as_ref());
                println!("Start tag: {}", tag);
                if tag == "artist" {
                    println!("Found <artist> tag");
                    let mut artist_buf = Vec::new();
                    // Write the <artist> start tag to the buffer
                    artist_buf.extend_from_slice(&buf);
                    let mut depth = 1;
                    buf.clear();
                    while depth > 0 {
                        let inner_event = reader.read_event_into(&mut buf)?;
                        match &inner_event {
                            Event::Start(e) => {
                                let start_name = e.name();
                                let start_tag = String::from_utf8_lossy(start_name.as_ref());
                                if start_tag == "artist" {
                                    depth += 1;
                                    println!("  Nested <artist> start tag, depth: {}", depth);
                                }
                                // Always buffer the code
                            }
                            Event::End(e) => {
                                let end_name = e.name();
                                let end_tag = String::from_utf8_lossy(end_name.as_ref());
                                println!("  End tag: {}", end_tag);
                                if end_tag == "artist" {
                                    depth -= 1;
                                    println!("  Closing <artist>, depth: {}", depth);
                                }
                                // Always buffer the code
                            }
                            Event::Eof => {
                                println!("  Reached EOF inside artist");
                                break;
                            }
                            _ => {
                                // Always buffer the code
                            }
                        }
                        artist_buf.extend_from_slice(&buf);
                        buf.clear();
                    }
                    println!("Buffered XML for artist:\n{}", String::from_utf8_lossy(&artist_buf));
                    match from_reader::<_, Artist>(artist_buf.as_slice()) {
                        Ok(artist) => {
                            count += 1;
                            println!("Parsed artist #{}: {:?}", count, artist);
                        }
                        Err(e) => {
                            println!("Error parsing artist: {}", e);
                        }
                    }
                }
            }
            Event::End(e) => {
                let name = e.name();
                let tag = String::from_utf8_lossy(name.as_ref());
                println!("End tag: {}", tag);
            }
            Event::Eof => {
                println!("Reached EOF");
                break;
            }
            _ => {
                println!("Other event: {:?}", event);
            }
        }
        buf.clear();
    }
    Ok(())
}