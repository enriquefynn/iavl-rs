use crate::iavl::*;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Graph {
  nodes: Vec<String>,
  edges: Vec<String>,
}

impl Graph {
  fn add_edge<K: Display, V: Display>(&mut self, from: &Node<K, V>, to: &Option<Box<Node<K, V>>>) {
    let fr = match from {
      Node::Inner { key, .. } => format!("\"{}\"", key),
      Node::Leaf { key, .. } => format!("\"{}l\"", key),
    };
    let to = match to {
      Some(to) => match to.as_ref() {
        Node::Inner { key, .. } => format!("\"{}\"", key),
        Node::Leaf { key, .. } => format!("\"{}l\"", key),
      },
      None => "\"empty\"".to_string(),
    };
    println!("FROM: {} TO: {}", fr, to);
    self.edges.push(format!("{} -- {}\n", fr, to));
  }

  fn add_node<K: Display, V: Display>(&mut self, node: &Node<K, V>) {
    let n = match node {
      Node::Inner { height, key, .. } => {
        format!("\"{}\" [label=\"{}\\nheight: {}\"]\n", key, key, height)
      }
      Node::Leaf { key, .. } => format!("\"{}l\" [shape=\"square\"; label=\"{}\"]\n", key, key),
    };
    self.nodes.push(n);
  }

  fn write(self, file: &mut std::fs::File) {
    for node in &self.nodes {
      file.write(node.as_bytes());
    }
    for edge in &self.edges {
      file.write(edge.as_bytes());
    }
  }

  fn new() -> Self {
    Graph {
      nodes: Vec::new(),
      edges: Vec::new(),
    }
  }
}

pub fn create_dot_graph<K: Display, V: Display>(filename: &String, iavl: IAVL<K, V>) {
  let path = Path::new(filename);

  let mut file = match File::create(&path) {
    Err(why) => panic!("Error creating file: {}", why.description()),
    Ok(file) => file,
  };

  let mut graph = Graph::new();
  file.write("strict graph{\n".as_bytes());
  write_link(&mut graph, &iavl.root);
  graph.write(&mut file);
  file.write("}\n".as_bytes());
}

fn write_link<K: Display, V: Display>(graph: &mut Graph, root: &Option<Box<Node<K, V>>>) {
  match root {
    Some(node) => {
      graph.add_node(node.as_ref());
      match node.as_ref() {
        Node::Leaf { .. } => {
          // graph.add_edge(node, n);
        }
        Node::Inner { left, right, .. } => {
          graph.add_edge(node, left);
          graph.add_edge(node, right);
          write_link(graph, &left);
          write_link(graph, &right);
        }
      }
    }
    None => {}
  }
}

#[test]
fn graphviz_tree() {
  let mut iavl = IAVL::new();
  iavl.insert(50, 50);
  iavl.insert(40, 40);
  iavl.insert(30, 30);
  iavl.insert(20, 20);
  iavl.insert(10, 10);
  iavl.insert(9, 9);
  iavl.insert(8, 8);
  let filename = "tree.dot".to_string();
  create_dot_graph(&filename, iavl);
  // assert_eq!(node.key, 1);
}
