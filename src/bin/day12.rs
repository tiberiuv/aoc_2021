use std::collections::{HashMap, HashSet};

type Id = String;

#[derive(Debug)]
struct Edge(Id, Id);

#[derive(Debug)]
struct Node {
    node_id: Id,
    kind: NodeKind,
}

impl<'a> From<&'a str> for Node {
    fn from(str: &'a str) -> Self {
        let node_kind = match str {
            "start" => NodeKind::Start,
            "end" => NodeKind::End,
            str => {
                if str == str.to_ascii_uppercase() {
                    NodeKind::BigCave
                } else {
                    NodeKind::SmallCave
                }
            }
        };
        Node {
            node_id: str.to_string(),
            kind: node_kind,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum NodeKind {
    BigCave,
    SmallCave,
    End,
    Start,
}

#[derive(Debug)]
struct Dag {
    edges: Vec<Edge>,
    nodes: HashMap<String, Node>,
}

impl Dag {
    const START: NodeKind = NodeKind::Start;
    const END: NodeKind = NodeKind::End;
    fn get_children(&self, current: Id) -> Vec<&Edge> {
        self.edges.iter().filter(|x| x.0 == current).collect()
    }
    fn get_valid_children(&self, current: Id, visited: &mut HashSet<Id>) -> Vec<Id> {
        self.get_children(current)
            .into_iter()
            .filter(|x| !visited.contains(&x.1))
            .map(|x| x.1.clone())
            .collect()
    }
}

impl<'a> From<&'a str> for Dag {
    fn from(str: &'a str) -> Self {
        let mut nodes = HashMap::new();
        let mut edges = Vec::new();
        str.lines().for_each(|x| {
            let mut split = x.splitn(2, '-');
            let parent = Node::from(split.next().expect("valid format"));
            let child = Node::from(split.next().expect("valid format"));
            let edge = Edge(parent.node_id.clone(), child.node_id.clone());
            edges.push(edge);
            nodes.insert(parent.node_id.clone(), parent);
            nodes.insert(child.node_id.clone(), child);
        });
        Self { nodes, edges }
    }
}

fn main() {
    let input = include_str!("../inputs/day12.txt");
    let graph = Dag::from(input);
}
