use std::env::args;
use std::path::PathBuf;

mod binarytree;
mod compressor;
mod decompressor;
mod error;
mod savebits;

use crate::compressor::compress;
use crate::decompressor::decompress;
use crate::error::ProgramError;

fn main() {
    if let Err(e) = run_program() {
        println!("ERROR: {}", e);
    }
}

fn run_program() -> Result<(), ProgramError> {
    let args = parse_arguments()?;
    let input = std::fs::read(args.input_file.clone()).map_err(|e| ProgramError::FileReadError(e, args.input_file.clone()))?;

    match args.program_type {
        ProgramType::Compress => {
            let compressed = compress(&input);
            let len1 = input.len();
            let len2 = compressed.len();
            println!(
                "{} bytes => {} bytes, {:.2} % compression ratio",
                len1,
                len2,
                len2 as f64 / len1 as f64 * 100.0
            );

            std::fs::write(args.output_file.clone(), compressed)
                .map_err(|e| ProgramError::FileWriteError(e, args.output_file.clone()))
        }
        ProgramType::Decompress => {
            let decompressed = decompress(input);
            std::fs::write(args.output_file.clone(), decompressed)
                .map_err(|e| ProgramError::FileWriteError(e, args.output_file.clone()))
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

fn parse_arguments() -> Result<ProgramArgs, ProgramError> {
    let mut args = args();
    args.next();
    let program_type = match args.next().as_deref() {
        Some("c") | Some("compress") => ProgramType::Compress,
        Some("d") | Some("decompress") => ProgramType::Decompress,
        _ => return Err(ProgramError::InvalidArgumentsError),
    };
    let input_file = match args.next() {
        Some(file_path) => PathBuf::from(file_path),
        None => return Err(ProgramError::InvalidArgumentsError),
    };
    let output_file = match args.next() {
        Some(file_path) => PathBuf::from(file_path),
        None => return Err(ProgramError::InvalidArgumentsError),
    };
    Ok(ProgramArgs {
        program_type,
        input_file,
        output_file,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_decompression() {
        let inputs = vec![
            b"".to_vec(),
            b"a".to_vec(),
            b"aaaa".to_vec(),
            b"hello world".to_vec(),
        ];

        for input in inputs {
            assert_eq!(decompress(compress(&input)), input);
        }
    }
}
