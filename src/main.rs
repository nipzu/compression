#![feature(binary_heap_drain_sorted)]

mod binarytree;
mod compressor;
mod decompressor;

fn main() {
    let input = b"hello world";
    println!("{:?}", compressor::compress(input));
}
