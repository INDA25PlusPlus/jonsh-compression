mod decode;
mod encode;

#[derive(Debug, Clone)]
pub enum Huffman {
    Leaf(u8, usize),
    Node(Box<Huffman>, Box<Huffman>, usize),
}

pub fn generate_huffman_tree(list: Vec<(u8, usize)>) -> Huffman {
    if list.len() == 1 {
        return Huffman::Leaf(list[0].0, list[0].1);
    } else {
        let left = generate_huffman_tree(vec![list[0]]);
        let right = generate_huffman_tree(list[1..].to_vec());
        return Huffman::Node(
            Box::new(left),
            Box::new(right),
            list.iter().map(|(_, x)| *x).sum(),
        );
    }
}
