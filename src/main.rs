// mod parser;
// mod exporter;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to your gzipped XML file
    let xml_gz_path = "/Users/c/data/discogsimport/discogs_20250901_artists.xml.gz";
    // Output directory for CSV files
    let out_dir = "/Users/c/data/rustexport/";

    parser::parse_artists_xml_to_csv(xml_gz_path, out_dir)
}
