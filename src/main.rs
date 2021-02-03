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
        eprintln!("ERROR: {}", e);
    }
}

fn run_program() -> Result<(), ProgramError> {
    let args = parse_arguments()?;

    let timer = if args.is_timed {
        Some(std::time::Instant::now())
    } else {
        None
    };

    let input = std::fs::read(args.input_file.clone())
        .map_err(|e| ProgramError::FileReadError(e, args.input_file.clone()))?;
    let input_len = input.len();
    if args.is_verbose {
        println!(
            "Succesfully read {} bytes from file {}",
            input_len,
            args.input_file.to_string_lossy()
        );
    }

    match args.program_type {
        ProgramType::Compress => {
            let compressed = compress(&input);
            let compressed_len = compressed.len();

            println!(
                "Compressed {} bytes to {} bytes with a {:.2} % compression ratio",
                input_len,
                compressed_len,
                compressed_len as f64 / input_len as f64 * 100.0
            );

            std::fs::write(args.output_file.clone(), compressed)
                .map_err(|e| ProgramError::FileWriteError(e, args.output_file.clone()))?;
            if args.is_verbose {
                println!(
                    "Succesfully wrote {} bytes to file {}",
                    compressed_len,
                    args.output_file.to_string_lossy()
                );
            }
        }
        ProgramType::Decompress => {
            let decompressed = decompress(input);
            let decompressed_len = decompressed.len();

            println!(
                "Decompressed {} bytes to {} bytes. The file had a {:.2} % compression ratio",
                input_len,
                decompressed_len,
                input_len as f64 / decompressed_len as f64 * 100.0
            );

            std::fs::write(args.output_file.clone(), decompressed)
                .map_err(|e| ProgramError::FileWriteError(e, args.output_file.clone()))?;

            if args.is_verbose {
                println!(
                    "Succesfully wrote {} bytes to file {}",
                    decompressed_len,
                    args.output_file.to_string_lossy()
                );
            }
        }
    }

    if let Some(start) = timer {
        println!("Done in {} ms", start.elapsed().as_millis());
    }

    Ok(())
}

enum ProgramType {
    Compress,
    Decompress,
}

struct ProgramArgs {
    program_type: ProgramType,
    input_file: PathBuf,
    output_file: PathBuf,
    is_verbose: bool,
    is_timed: bool,
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
    let mut is_verbose = false;
    let mut is_timed = false;
    while let Some(arg) = args.next().as_deref() {
        match arg {
            "-v" => is_verbose = true,
            "-t" => is_timed = true,
            _ => eprintln!(
                "WARNING: Argumement {} was not recognized and was ignored",
                arg
            ),
        }
    }
    Ok(ProgramArgs {
        program_type,
        input_file,
        output_file,
        is_verbose,
        is_timed,
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
