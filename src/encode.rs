use std::fs;

use crate::Huffman;

pub fn generate_list(path: String) -> Vec<(u8, usize)> {
    let mut buf = Vec::new();
    buf = fs::read(path).unwrap();
    let mut list: Vec<(u8, usize)> = Vec::new();
    for i in buf {
        if list.iter().any(|(x, _)| *x == i) {
            let index = list.iter().position(|(x, _)| *x == i).unwrap();
            list[index].1 += 1;
        } else {
            list.push((i, 1));
        }
    }
    list.sort_by(|a, b| b.1.cmp(&a.1));
    return list;
}

pub fn generate_huffman_string(huffman: Huffman) -> String {
    let buf = walk(huffman, String::new());
    return buf;
}

pub fn walk(huffman: Huffman, prefix: String) -> String {
    let mut prefix = prefix;
    match huffman {
        Huffman::Leaf(i, _) => {
            return format!("{}:{}\n", prefix, char::from_u32(i as u32).unwrap());
        }
        Huffman::Node(left, right, _) => {
            return format!(
                "{}{}",
                walk(*left, format!("{}0", prefix)),
                walk(*right, format!("{}1", prefix))
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_generate_list() {
        let list = generate_list("test.txt".to_string());
        assert_eq!(list, vec![(66, 2), (65, 1)]);
    }

    #[test]
    fn test_generate_huffman_tree() {
        let list = generate_list("test.txt".to_string());
        let tree = generate_huffman_tree(list);
        println!("HuffMan: {:?}", tree);
    }

    #[test]
    fn test_generate_huffman_string() {
        let list = generate_list("test.txt".to_string());
        println!("list: {:?}", list);
        let tree = generate_huffman_tree(list);
        println!("Tree: {:?}", tree);
        println!("Huffman: {:?}", generate_huffman_string(tree));
    }
}
