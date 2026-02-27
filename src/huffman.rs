use std::{collections::HashMap, fs};

pub fn encode(input: &[u8], table: &HashMap<u8, Vec<bool>>) -> (Vec<u8>, u8) {
    let mut writer = Bitwriter::default();
    for &b8 in input {
        let code = &table[&b8];
        for &b1 in code {
            writer.push(b1);
        }
    }
    writer.brrrrrrrrrrrrrrrr()
}

pub fn decode(tree: &Tree, compressed: Vec<u8>, vb: u8) -> Vec<u8> {
    let mut reader = Bitreader::new(compressed, vb);
    let mut out = Vec::new();
    let mut cur = tree;
    while let Some(b1) = reader.ne_b() {
        cur = match cur {
            Tree::Leaf(_, _) => unreachable!(),
            Tree::Node(_, left, right) => {
                if b1 {
                    right
                } else {
                    left
                }
            }
        };
        if let Tree::Leaf(_, b8) = cur {
            out.push(*b8);
            cur = tree;
        }
    }
    out
}

#[derive(Debug, Clone)]
pub enum Tree {
    Leaf(u64, u8),
    Node(u64, Box<Tree>, Box<Tree>),
}

impl Tree {
    pub fn freq(&self) -> u64 {
        match self {
            Tree::Leaf(x, _) => *x,
            Tree::Node(x, _, _) => *x,
        }
    }

    pub fn serialize(&self, out: &mut Vec<u8>) {
        match self {
            Tree::Leaf(_, b) => {
                out.push(1);
                out.push(*b);
            }
            Tree::Node(_, left, right) => {
                out.push(0);
                left.serialize(out);
                right.serialize(out);
            }
        }
    }

    pub fn deserialize(input: &mut &[u8]) -> Tree {
        let tag = input[0];
        *input = &input[1..];
        match tag {
            1 => {
                let b = input[0];
                *input = &input[1..];
                Tree::Leaf(0, b)
            }
            0 => {
                let left = Box::new(Tree::deserialize(input));
                let right = Box::new(Tree::deserialize(input));
                Tree::Node(0, left, right)
            }
            _ => panic!("Stop Writing shit code and go to sleep dumbasssssssss"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Huffman {
    root: Tree,
}

impl Huffman {
    pub fn parse_file(data: Vec<u8>) -> Vec<Tree> {
        let mut list: Vec<(u64, u8)> = Vec::new();
        for i in data.iter() {
            if list.iter().any(|(_, x)| *x == *i) {
                let index = list.iter().position(|(_, x)| *x == *i).unwrap();
                list[index].0 += 1;
            } else {
                list.push((1, *i));
            }
        }
        list.sort_by_key(|(x, _)| *x);
        list.reverse();
        let mut buf: Vec<Tree> = Vec::new();
        for i in list {
            buf.push(Tree::Leaf(i.0, i.1))
        }
        buf
    }

    pub fn build_tree(list: &mut Vec<Tree>) -> Huffman {
        while list.len() > 1 {
            let t1 = list.pop().unwrap();
            let t2 = list.pop().unwrap();
            list.push(Tree::Node(
                t1.freq() + t2.freq(),
                Box::new(t1),
                Box::new(t2),
            ));
            list.sort_by_key(|x| x.freq());
            list.reverse();
        }
        if list.len() < 1 {
            panic!("Fix your shit code");
        }
        Huffman {
            root: list.pop().unwrap(),
        }
    }

    pub fn walk(tree: &Tree, path: &mut Vec<bool>, table: &mut HashMap<u8, Vec<bool>>) {
        match tree {
            Tree::Leaf(_, b) => {
                table.insert(*b, path.clone());
            }
            Tree::Node(_, left, right) => {
                path.push(false);
                Self::walk(left, path, table);
                path.pop();
                path.push(true);
                Self::walk(right, path, table);
                path.pop();
            }
        }
    }

    pub fn build_code(&self) -> HashMap<u8, Vec<bool>> {
        let mut hasch = HashMap::new();
        let mut path = Vec::new();
        Self::walk(&self.root, &mut path, &mut hasch);
        hasch
    }
}

#[derive(Default)]
pub struct Bitwriter {
    buf: Vec<u8>,
    cur: u8,
    used: u8,
}

pub struct Bitreader {
    data: Vec<u8>,
    b8i: usize,
    b1i: u8,
    last: u8,
}

impl Bitreader {
    pub fn new(data: Vec<u8>, last: u8) -> Self {
        Bitreader {
            data,
            b8i: 0,
            b1i: 0,
            last,
        }
    }

    pub fn ne_b(&mut self) -> Option<bool> {
        if self.b8i >= self.data.len() {
            return None;
        }
        if self.b8i == self.data.len() - 1 && self.b1i >= self.last {
            return None;
        }
        let b8 = self.data[self.b8i];
        let b1 = (b8 & (1 << (7 - self.b1i))) != 0;
        self.b1i += 1;
        if self.b1i == 8 {
            self.b1i = 0;
            self.b8i += 1;
        }

        Some(b1)
    }
}

impl Bitwriter {
    pub fn push(&mut self, b: bool) {
        self.cur <<= 1;
        if b {
            self.cur |= 1;
        }
        self.used += 1;

        if self.used == 8 {
            self.buf.push(self.cur);
            self.cur = 0;
            self.used = 0;
        }
    }

    pub fn brrrrrrrrrrrrrrrr(mut self) -> (Vec<u8>, u8) {
        if self.used > 0 {
            self.cur <<= 8 - self.used;
            self.buf.push(self.cur);
        }
        let vb = if self.used == 0 { 8 } else { self.used };
        (self.buf, vb)
    }
}

#[cfg(test)]
mod test {
    use crate::huffman::{self, Huffman, Tree};

    #[test]
    fn test_build_tree() {
        let data: Vec<u8> = "Hello this is a small test file with little words but big enough that the test produces some result. A fun fact: The sentence \"Buffalo buffalo Buffalo buffalo buffalo buffalo Buffalo buffalo\" is grammatically correct".chars().map(|x| x as u8).collect();
        let mut list = Huffman::parse_file(data);
        let tree = Huffman::build_tree(&mut list);
        let code = Huffman::build_code(&tree);
        println!("{:#?}", code);
    }

    #[test]
    fn test_sedeserialize() {
        let tree = Tree::Node(
            0,
            Box::new(Tree::Leaf(0, 1)),
            Box::new(Tree::Node(
                0,
                Box::new(Tree::Leaf(0, 2)),
                Box::new(Tree::Leaf(0, 3)),
            )),
        );
        let mut ser = Vec::new();
        tree.serialize(&mut ser);
        let deser = Tree::deserialize(&mut &ser[..]);
        assert_eq!(format!("{:?}", tree), format!("{:?}", deser));
    }
}
