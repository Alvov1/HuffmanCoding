use std::collections::HashMap;

struct Node {
    freq: i32,
    ch: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn new(freq: i32, ch: Option<char>) -> Node {
        Node{ freq, ch, left: None, right: None }
    }
}

struct HuffmanTree {
    root: Box<Node>,
    codes: HashMap<char, String>
}
impl HuffmanTree {
    fn generate(pattern: &str) -> HuffmanTree {
        let frequencies = HuffmanTree::frequency(pattern);

        let mut nodes: Vec<Box<Node>> = Vec::with_capacity(frequencies.len());
        for (ch, freq) in &frequencies {
            nodes.push(Box::new(Node::new(*freq, Some(*ch))));
        }

        if nodes.len() == 1 {
            let internal_root = nodes.pop().unwrap();
            let mut root = Box::new(Node::new(internal_root.freq, None));
            root.right = Some(internal_root); nodes.push(root);
        } else {
            while nodes.len() > 1 {
                nodes.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
                let left = nodes.pop().unwrap();
                let right = nodes.pop().unwrap();
                let common_freq = left.freq + right.freq;
                let mut common_root = Box::new(Node::new(common_freq, None));
                common_root.left = Some(left); common_root.right = Some(right); nodes.push(common_root);
            }
        }

        let mut tree: HuffmanTree = HuffmanTree {
            root: nodes.pop().unwrap(),
            codes: HashMap::new()
        };

        HuffmanTree::assign_codes(&tree.root, &mut tree.codes, "".to_string());

        tree
    }

    fn encode(&self, pattern: &str) -> String {
        let mut result = String::new();
        for ch in pattern.chars() {
            result += self.codes.get(&ch).unwrap();
        }
        result
    }
    fn frequency(s: &str) -> HashMap<char, i32> {
        let mut h = HashMap::new();
        for ch in s.chars() {
            let counter = h.entry(ch).or_insert(0);
            *counter += 1;
        }
        h
    }
    fn assign_codes(node: &Box<Node>, map: &mut HashMap<char, String>, s: String) {
        if let Some(ch) = node.ch {
            map.insert(ch, s);
        } else {
            if let Some(ref left) = node.left {
                HuffmanTree::assign_codes(left, map, s.clone() + "1");
            }
            if let Some(ref right) = node.right {
                HuffmanTree::assign_codes(right, map, s.clone() + "0");
            }
        }
    }

    fn decode(codes: &HashMap<char, &str>, mut value: String) -> String {
        let mut result = String::new();
        while value.len() > 0 {
            for (ch, code) in codes {
                if value.starts_with(code) {
                    result += &ch.to_string();
                    let (_, new_value) = value.split_at(code.len());
                    value = new_value.to_string();
                }
            }
        }
        result
    }
}

fn main() {
    /* Option 1: Generate tree from input string, than encode it. */
    let message = "abacabad";
    let tree = HuffmanTree::generate(&message);
    println!("{}", tree.encode(&message));

    /* Option 2: Decode from symbol codes and encoded message. */
    let codes: HashMap<_, _> = [('a', "0"), ('b', "10"), ('c', "110"), ('d', "111")].into_iter().collect();
    let encoded= "01001100100111".to_string();
    println!("{}", HuffmanTree::decode(&codes, encoded));
}
