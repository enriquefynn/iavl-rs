use std::cmp;
use std::cmp::Ordering;

pub enum Node<K, V> {
  Leaf {
    key: K,
    value: V,
    hash: Option<[u128; 2]>,
    dirty: bool, // Used to lazily calculate hash
  },
  Inner {
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    key: K,
    hash: Option<[u128; 2]>,
    height: u8,
    dirty: bool, // Used to lazily calculate hash
  },
}

pub struct IAVL<K, V> {
  pub root: Option<Box<Node<K, V>>>,
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
          Some(l) => {
            println!("LEFT:");
            l.print();
          }
          None => {}
        }
        match right {
          Some(r) => {
            println!("RIGHT:");
            r.print()
          }
          None => {}
        }
      }
    }
  }

  pub fn new_leaf(key: K, value: V) -> Self {
    Node::Leaf {
      key: key,
      value: value,
      hash: None,
      dirty: true,
    }
  }

  fn insert_in_child(
    root: Option<Box<Node<K, V>>>,
    new_key: K,
    new_value: V,
  ) -> Option<Box<Node<K, V>>>
  where
    K: Ord + Copy,
  {
    Some(match root {
      Some(node) => Node::insert(node, new_key, new_value),
      None => Box::new(Node::new_leaf(new_key, new_value)),
    })
  }

  pub fn insert(mut root: Box<Node<K, V>>, new_key: K, new_value: V) -> Box<Node<K, V>>
  where
    K: Ord + Copy,
  {
    match *root {
      Node::Inner {
        key,
        ref mut right,
        ref mut left,
        ..
      } => {
        if new_key < key {
          *left = Node::insert_in_child(left.take(), new_key, new_value)
        } else {
          *right = Node::insert_in_child(right.take(), new_key, new_value)
        }
      }
      Node::Leaf { key, .. } => {
        if new_key < key {
          root = Box::new(Node::Inner {
            key: key,
            left: Some(Box::new(Node::new_leaf(new_key, new_value))),
            right: Some(root),
            hash: None,
            height: 1,
            dirty: true,
          });
        } else {
          root = Box::new(Node::Inner {
            key: new_key,
            right: Some(Box::new(Node::new_leaf(new_key, new_value))),
            left: Some(root),
            hash: None,
            height: 1,
            dirty: true,
          });
        }
      }
    }
    Node::update_height(&mut root);
    Node::balance(root)
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

  fn update_height(root: &mut Box<Node<K, V>>) {
    match root.as_mut() {
      Node::Inner {
        ref left,
        ref right,
        ref mut height,
        ref mut dirty,
        ..
      } => {
        *height = cmp::max(Node::height(left), Node::height(right)) + 1;
        *dirty = true;
      }
      Node::Leaf { dirty, .. } => {
        *dirty = true;
      }
    }
  }

  pub fn update_hash(root: &mut Box<Node<K, V>>) -> Option<[u128; 2]> {
    match root.as_mut() {
      Node::Leaf { dirty, .. } => {
        // update hash
        Some([0u128, 1u128])
      }
      Node::Inner {
        ref dirty,
        ref mut left,
        ref mut right,
        ..
      } => {
        if !dirty {
          return None;
        }
        let h_left = match left.as_mut() {
          Some(node) => Node::update_hash(node),
          None => None,
        };
        let r_right = match right.as_mut() {
          Some(node) => Node::update_hash(node),
          None => None,
        };
        Some([0u128, 1u128])
      }
    }
  }

  fn rotate_right(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    match *root {
      Node::Leaf { .. } => unreachable!("Should not rotate leaf"),
      Node::Inner {
        left: ref mut root_left,
        ref key,
        ..
      } => {
        let mut r = root_left.take().unwrap();
        match *r {
          Node::Leaf { .. } => unreachable!("Broken algorithm"),
          Node::Inner { ref mut right, .. } => {
            *root_left = right.take();
            Node::update_height(&mut root);
            *right = Some(root);
            Node::update_height(&mut r);
          }
        }
        return r;
      }
    }
  }

  fn rotate_right_left(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    println!("ROTATE RL");
    match *root {
      Node::Leaf { .. } => unreachable!("Should not rotate leaf"),
      Node::Inner {
        right: ref mut root_right,
        ..
      } => {
        let mut r = root_right.take().unwrap();
        match r.as_mut() {
          Node::Leaf { .. } => unreachable!("Broken algorithm"),
          Node::Inner { right, left, .. } => {
            if Node::get_height(left) > Node::get_height(right) {
              let rotated_root = Node::rotate_right(r);
              *root_right = Some(rotated_root);
              Node::update_height(&mut root);
            } else {
              // Give back from take
              *root_right = Some(r);
            }
          }
        }
        Node::rotate_left(root)
      }
    }
  }

  fn rotate_left(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    match *root {
      Node::Leaf { .. } => unreachable!("Should not rotate leaf"),
      Node::Inner {
        right: ref mut root_right,
        ..
      } => {
        let mut r = root_right.take().unwrap();
        match *r {
          Node::Leaf { .. } => unreachable!("Broken algorithm"),
          Node::Inner { ref mut left, .. } => {
            *root_right = left.take();
            Node::update_height(&mut root);
            *left = Some(root);
            Node::update_height(&mut r);
          }
        }
        return r;
      }
    }
  }

  fn rotate_left_right(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    match *root {
      Node::Leaf { .. } => unreachable!("Should not rotate leaf"),
      Node::Inner {
        left: ref mut root_left,
        ..
      } => {
        let mut l = root_left.take().unwrap();
        match l.as_mut() {
          Node::Leaf { .. } => unreachable!("Broken algorithm"),
          Node::Inner { left, right, .. } => {
            if Node::get_height(right) > Node::get_height(left) {
              let rotated_root = Node::rotate_left(l);
              *root_left = Some(rotated_root);
              Node::update_height(&mut root);
            } else {
              // Give back from take
              *root_left = Some(l);
            }
          }
        }
        println!("JUST");
        Node::rotate_right(root)
      }
    }
  }

  fn get_height(root: &Option<Box<Node<K, V>>>) -> u8 {
    match root.as_ref() {
      None => 0,
      Some(node) => match node.as_ref() {
        Node::Leaf { .. } => 0,
        Node::Inner { height, .. } => *height,
      },
    }
  }

  fn height_difference(root: &Box<Node<K, V>>) -> i8 {
    match root.as_ref() {
      Node::Leaf { .. } => 0,
      Node::Inner { left, right, .. } => {
        let l = Node::get_height(left);
        let r = Node::get_height(right);
        (l as i8) - (r as i8)
      }
    }
  }

  pub fn balance(root: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let height_diff = Node::height_difference(&root);
    if height_diff >= -1 && height_diff <= 1 {
      return root;
    }
    match height_diff {
      2 => Node::rotate_left_right(root),
      -2 => Node::rotate_right_left(root),
      _ => unreachable!(),
    }
  }
}

impl<'a, K: Ord, V> Node<K, V> {
  pub fn search(search_key: &K, root: &'a Box<Node<K, V>>) -> Option<(&'a K, &'a V)> {
    match root.as_ref() {
      Node::Leaf { key, value, .. } => {
        if key == search_key {
          Some((&key, &value))
        } else {
          None
        }
      }
      Node::Inner {
        key, left, right, ..
      } => {
        if search_key < key {
          left
            .as_ref()
            .map_or(None, |node| Node::search(search_key, node))
        } else {
          right
            .as_ref()
            .map_or(None, |node| Node::search(search_key, node))
        }
      }
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
  {
    match self.root.take() {
      None => {
        self.root = Some(Box::new(Node::new_leaf(new_key, new_value)));
      }
      Some(root) => {
        self.root = Some(Node::insert(root, new_key, new_value));
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn construct_tree() {
    let mut iavl = IAVL::new();
    iavl.insert(4, 4);
  }

  #[test]
  fn search() {
    let mut iavl = IAVL::new();
    for i in 0..10 {
      iavl.insert(i, i);
    }
    let root = &iavl.root.unwrap();
    let s = Node::search(&11, root);
    match s {
      None => {}
      Some(_) => assert!(false),
    }
    let s = Node::search(&4, root);
    match s {
      None => assert!(false),
      Some(_) => {}
    }
  }
}
