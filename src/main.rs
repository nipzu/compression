use std::env::args;
use std::path::PathBuf;

mod binarytree;
mod compressor;
mod decompressor;
mod savebits;

fn main() {
    let args = parse_arguments();

    match args.program_type {
        ProgramType::Compress => {
            let input = std::fs::read(args.input_file).expect("TODO");
            let compressed = compressor::compress(&input);
            let len1 = input.len();
            let len2 = compressed.len();
            println!(
                "{} bytes => {} bytes, {:.2} % compression ratio",
                len1,
                len2,
                len2 as f64 / len1 as f64 * 100.0
            );

            std::fs::write(args.output_file, compressed).expect("Error while writing to file");
        }
        ProgramType::Decompress => {
            let input = std::fs::read(args.input_file).expect("TODO");
            let decompressed = decompressor::decompress(input);
            std::fs::write(args.output_file, decompressed).expect("Error while writing to file");
        }
    }
}

enum ProgramType {
    Compress,
    Decompress,
}

struct ProgramArgs {
    program_type: ProgramType,
    input_file: PathBuf,
    output_file: PathBuf,
}

fn parse_arguments() -> ProgramArgs {
    let mut args = args();
    args.next();
    let program_type = match args.next().as_deref() {
        Some("c") | Some("compress") => ProgramType::Compress,
        Some("d") | Some("decompress") => ProgramType::Decompress,
        _ => panic!("TODO"),
    };
    let input_file = match args.next() {
        Some(file_path) => PathBuf::from(file_path),
        None => panic!("TODO"),
    };
    let output_file = match args.next() {
        Some(file_path) => PathBuf::from(file_path),
        None => panic!("TODO"),
    };
    ProgramArgs {
        program_type,
        input_file,
        output_file,
    }
}
