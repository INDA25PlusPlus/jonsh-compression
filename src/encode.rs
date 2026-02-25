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

// Mitt träd ser ut såhär utifrån erat exempel från slide 19
//     6        | Istället för: |      6
//   /   \      |               |    /   \
// a:3    3     |               |  a:3    3
//      /   \   |               |       /   \
//    b:2    1  |               |     b:2   c:1
//          /   |               |
//        c:1   |               |

pub fn generate_huffman_string(huffman: Huffman) -> String {
    let mut buf = String::new();
    match huffman {
        Huffman::Leaf(i, j) => {
            buf.push_str(format!("0:{}\n", char::from_u32(i as u32).unwrap()).as_str());
        }
        Huffman::Node(huffman, huffman1, _) => {
            buf.push_str(format!("1{}", generate_huffman_string(*huffman1)).as_str());
        }
    }
    buf
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
        println!("Huffman: {:?}", generate_huffman_string(tree));
    }
}
