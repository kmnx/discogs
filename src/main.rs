// mod parser;
// mod exporter;
mod parser;

fn main() -> std::io::Result<()> {
    // Path to your gzipped XML file
    let xml_gz_path = "/Users/c/data/discogsimport/discogs_20250901_artists.xml.gz";
    // Output directory for CSV files
    let out_dir = "/Users/c/data/rustexport/";

    export_artists_xml_to_csv(xml_gz_path, out_dir)
}

pub fn export_artists_xml_to_csv<P: AsRef<Path>>(xml_gz_path: P, out_dir: P) -> std::io::Result<()> {
    let file = File::open(xml_gz_path)?;
    let gz = GzDecoder::new(file);
    let mut reader = Reader::from_reader(BufReader::new(gz));
    let out_dir = out_dir.as_ref();