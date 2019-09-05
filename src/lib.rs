use std::cmp::Ordering;
enum Node<K, V> {
    Leaf {
        key: K,
        value: V,
        hash: Option<[u8; 256]>,
    },
    Inner {
        left: Option<Box<Node<K, V>>>,
        right: Option<Box<Node<K, V>>>,
        key: K,
        hash: Option<[u8; 256]>,
    },
}

struct IAVL<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    pub fn print(self)
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        match self {
            Node::Leaf { key, value, .. } => {
                println!("Leaf: {} {}", key, value);
            }
            Node::Inner {
                key, left, right, ..
            } => {
                println!("Inner: {}", key);
                match left {
                    Some(l) => l.print(),
                    _ => {}
                }
                match right {
                    Some(r) => r.print(),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    pub fn new(key: K, value: V) -> Self
    where
        K: Copy,
    {
        Node::Inner {
            key: key,
            hash: None,
            left: None,
            right: Some(Box::new(Node::Leaf {
                key: key,
                value: value,
                hash: None,
            })),
        }
    }

    pub fn insert(root: &mut Option<Box<Node<K, V>>>, new_key: K, new_value: V)
    where
        K: Ord + Copy,
    {
        match root {
            None => {
                *root = Some(Box::new(Node::Leaf {
                    key: new_key,
                    value: new_value,
                    hash: None,
                }));
            }
            Some(r) => match **r {
                Node::Inner {
                    key,
                    ref mut right,
                    ref mut left,
                    ..
                } => {
                    if new_key < key {
                        Node::insert(left, new_key, new_value)
                    } else {
                        Node::insert(right, new_key, new_value)
                    }
                    return;
                }
                Node::Leaf { key, .. } => {
                    if new_key < key {
                        *root = Some(Box::new(Node::Inner {
                            key: key,
                            left: Some(Box::new(Node::Leaf {
                                key: new_key,
                                value: new_value,
                                hash: None,
                            })),
                            right: root.take(),
                            hash: None,
                        }));
                    } else {
                        *root = Some(Box::new(Node::Inner {
                            key: new_key,
                            right: Some(Box::new(Node::Leaf {
                                key: new_key,
                                value: new_value,
                                hash: None,
                            })),
                            left: root.take(),
                            hash: None,
                        }));
                    }
                }
            },
        }
    }
}

impl<K, V> IAVL<K, V> {
    pub fn insert(&mut self, new_key: K, new_value: V)
    where
        K: Ord + Copy,
    {
        Node::insert(&mut self.root, new_key, new_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_node() {
        let node = Node::new(1, 1);
        match node {
            Node::Leaf { key, .. } => assert_eq!(key, 1),
            _ => panic!(""),
        }
    }

    #[test]
    fn construct_tree() {
        let mut iavl = IAVL {
            root: Some(Box::new(Node::new(8, 8))),
        };
        iavl.insert(4, 4);
        iavl.insert(3, 3);
        iavl.insert(11, 11);
        iavl.root.unwrap().print();
        // assert_eq!(node.key, 1);
    }
}
