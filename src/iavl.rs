use std::cmp;
use std::hash::{Hash, Hasher};

pub enum Node<K, V> {
  Leaf {
    key: K,
    value: V,
    hash: Option<[u128; 2]>,
  },
  Inner {
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    key: K,
    hash: Option<[u128; 2]>,
    height: u8,
  },
}

impl<K, V> Hash for Node<K, V>
where
  V: Hash,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Node::Inner { .. } => {}
      Node::Leaf { hash, value, .. } => {
        value.hash(state);
      }
    }
    // self.id.hash(state);
    // self.phone.hash(state);
  }
}

pub struct IAVL<K, V> {
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
        key,
        left,
        right,
        height,
        ..
      } => {
        println!("Inner: key: {}, height: {}", key, height);
        match left {
          Some(l) => l.print(),
          _ => {}
        }
        match right {
          Some(r) => r.print(),
          _ => {}
        }
      }
    }
  }

  pub fn new_leaf(key: K, value: V) -> Self {
    Node::Leaf {
      key: key,
      value: value,
      hash: None,
    }
  }

  pub fn insert(root: &mut Option<Box<Node<K, V>>>, new_key: K, new_value: V)
  where
    K: Ord + Copy,
    V: Hash,
  {
    match root {
      None => {
        *root = Some(Box::new(Node::new_leaf(new_key, new_value)));
      }
      Some(r) => match **r {
        Node::Inner {
          key,
          ref mut right,
          ref mut left,
          ..
        } => {
          let r_l;
          if new_key < key {
            r_l = left;
          } else {
            r_l = right;
          }
          Node::insert(r_l, new_key, new_value);
          Node::update_height_hash(r_l);
          Node::balance(r_l);
        }
        Node::Leaf { key, .. } => {
          if new_key < key {
            *root = Some(Box::new(Node::Inner {
              key: key,
              left: Some(Box::new(Node::new_leaf(new_key, new_value))),
              right: root.take(),
              hash: None,
              height: 1,
            }));
          } else {
            *root = Some(Box::new(Node::Inner {
              key: new_key,
              right: Some(Box::new(Node::new_leaf(new_key, new_value))),
              left: root.take(),
              hash: None,
              height: 1,
            }));
          }
        }
      },
    }
  }

  fn height(root: &Option<Box<Node<K, V>>>) -> u8 {
    match root {
      Some(node) => match node.as_ref() {
        Node::Inner { height, .. } => *height,
        Node::Leaf { .. } => 0,
      },
      None => 0,
    }
  }

  fn update_height_hash(root: &mut Option<Box<Node<K, V>>>)
  where
    V: Hash,
  {
    match root {
      Some(node) => match node.as_mut() {
        Node::Inner {
          ref left,
          ref right,
          ref mut height,
          ..
        } => {
          *height = cmp::max(Node::height(left), Node::height(right)) + 1;
        }
        Node::Leaf { .. } => {}
      },
      None => {}
    }
  }

  fn rotate_right(root: &mut Option<Box<Node<K, V>>>) {
    match root {
      Some(node) => match node.as_mut() {
        Node::Inner {
          left: ref mut root_left,
          ..
        } => {
          match *root_left.take().unwrap() {
            Node::Inner { ref mut right, .. } => {
              *root_left = right.take();
              // update height root_left
              *right = root.take();
              // update height right
            }
            Node::Leaf { .. } => {}
          }
        }
        Node::Leaf { .. } => {}
      },
      None => {}
    }
  }

  fn rotate_left(root: &mut Option<Box<Node<K, V>>>) {
    match root {
      Some(node) => match node.as_mut() {
        Node::Inner {
          right: ref mut root_right,
          ..
        } => {
          match *root_right.take().unwrap() {
            Node::Inner { ref mut left, .. } => {
              *root_right = left.take();
              // update height root_right
              *left = root.take();
              // update height left
            }
            Node::Leaf { .. } => {}
          }
        }
        Node::Leaf { .. } => {}
      },
      None => {}
    }
  }

  pub fn balance(root: &mut Option<Box<Node<K, V>>>)
  where
    K: Ord + Copy,
  {
    match root {
      Some(node) => match node.as_mut() {
        Node::Inner { left, right, .. } => {}
        Node::Leaf { .. } => {}
      },
      None => {}
    }
  }
}

impl<K, V> IAVL<K, V> {
  pub fn new() -> Self {
    return IAVL { root: None };
  }
  pub fn insert(&mut self, new_key: K, new_value: V)
  where
    K: Ord + Copy,
    V: Hash,
  {
    Node::insert(&mut self.root, new_key, new_value);
    Node::update_height_hash(&mut self.root);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn construct_tree() {
    let mut iavl = IAVL::new();
    iavl.insert(4, 4);
    iavl.insert(3, 3);
    iavl.insert(10, 10);
    iavl.insert(20, 20);
    iavl.insert(11, 11);
    iavl.root.unwrap().print();
    // assert_eq!(node.key, 1);
  }
}
