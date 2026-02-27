mod huffman;
use std::fs;

use crate::huffman::{Huffman, encode, write_to_file_finaly_done_finished};

fn main() {
    let input = fs::read("testfile.txt").unwrap();
    let mut löööv = Huffman::parse_file(input.clone());
    let huffman = Huffman::build_tree(&mut löööv);
    let hasch = huffman.build_code();
    let (compressssssed, vb) = encode(&input, &hasch);
    write_to_file_finaly_done_finished("output.huff", &huffman.root, compressssssed, vb).unwrap();
}
