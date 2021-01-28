mod binarytree;
mod compressor;
mod decompressor;
mod savebits;

fn main() {
    let input = b"hello world";

    for byte in input {
        print!("{}", *byte as char);
    }
    println!();
    println!();

    let compressed = compressor::compress(input);
    for byte in compressed.clone() {
        print!(
            "{} ",
            format!("{:08b}", byte).chars().rev().collect::<String>()
        );
    }
    println!();

    let len1 = input.len();
    let len2 = compressed.len();

    println!();
    println!(
        "{} bytes => {} bytes, {:.2} % compression ratio",
        len1,
        len2,
        len2 as f64 / len1 as f64 * 100.0
    );

    println!();
    let decompressed = decompressor::decompress(compressed);

    for byte in decompressed {
        print!("{}", byte as char);
    }
    println!();
}
