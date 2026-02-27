use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
pub enum Tree {
    Leaf(u64, u8),
    Node(u64, Box<Tree>, Box<Tree>),
}

impl Tree {
    fn freq(&self) -> u64 {
        match self {
            Tree::Leaf(x, _) => *x,
            Tree::Node(x, _, _) => *x,
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

    pub fn build_tree(list: Vec<Tree>) -> Huffman {
        let mut list = list;
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
        let mut path = path;
        let mut table = table;
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

#[cfg(test)]
mod test {
    use crate::huffman::{self, Huffman, Tree};

    #[test]
    fn test_build_tree() {
        let data: Vec<u8> = "Hello this is a small test file with little words but big enough that the test produces some result. A fun fact: The sentence \"Buffalo buffalo Buffalo buffalo buffalo buffalo Buffalo buffalo\" is grammatically correct".chars().map(|x| x as u8).collect();
        let list = Huffman::parse_file(data);
        let tree = Huffman::build_tree(list);
        let code = Huffman::build_code(&tree);
        println!("{:#?}", code);
    }
}
