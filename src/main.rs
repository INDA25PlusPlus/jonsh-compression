mod huffman;
use std::fs;

use crate::huffman::Huffman;

fn main() {
    let data: Vec<u8> = fs::read("testfile.txt")?;
}
