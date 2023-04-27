use flate2::read::GzDecoder;
use std::io::Read;

use base64::{alphabet, engine, read};

fn main() -> std::io::Result<()> {
    let cmd = clap::Command::new("eso-tool")
        .bin_name("eso-tool")
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("decode")
            .about("decode text content")
            .arg(clap::Arg::new("file")
                .long("file").help("Required configuration file to use")
                .value_parser(clap::value_parser!(std::path::PathBuf)))
            .arg(clap::Arg::new("prefix")
                .long("prefix").help("find prefix for start parser")
                .value_parser(clap::value_parser!(String))));

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("decode", matches)) => {
            let file_path = matches.get_one::<std::path::PathBuf>("file").expect("get failed");
            println!("{:?}", file_path);

            let prefix = matches.get_one::<String>("prefix").expect("get failed");
            println!("{:?}", prefix);

            let mut f = std::fs::File::open(file_path)?;
            let mut out = String::new();
            f.read_to_string(&mut out).expect("read failed");
            let index = out.find(prefix).expect("find @ failed");
            let s = decode(&mut out[index+prefix.len()..].as_bytes());
            let pretty = format_to_json(&s).expect("to json failed");
            println!("=============");
            println!("{}", pretty);
        },
        _ => unreachable!("clap should ensure we don't get here"),
    };
    
    Ok(())
}

fn decode<R: std::io::Read>(context: &mut R) -> String {
    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD,engine::general_purpose::PAD);
    let mut decoder = read::DecoderReader::new(context, &engine);
    let mut out = Vec::new();
    let size = decoder.read_to_end(&mut out).unwrap();
    let mut d = GzDecoder::new(&out[0..size]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    s
}

fn format_to_json(data: &str) -> serde_json::Result<String> {
    let value: serde_json::Value = serde_json::from_str(data)?;
    return serde_json::to_string_pretty(&value);
}