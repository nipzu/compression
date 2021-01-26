mod binarytree;
mod compressor;
mod decompressor;

fn main() {
    let input = b"hahahaha";
    for byte in compressor::compress(input) {
        print!("{} ", format!("{:08b}", byte).chars().rev().collect::<String>());
    }
    println!();
}
