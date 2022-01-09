use itertools::Itertools;
use std::collections::HashMap;

#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy)]
pub enum TreeNodeType {
    Start,
    End,
    Big(&'static str),
    Small(&'static str),
}

impl TreeNodeType {
    pub fn from_text(s: &'static str) -> TreeNodeType {
        if s == "start" {
            TreeNodeType::Start
        } else if s == "end" {
            TreeNodeType::End
        } else if s.eq(s.to_uppercase().as_str()) {
            TreeNodeType::Big(s)
        } else {
            TreeNodeType::Small(s)
        }
    }
}

pub struct AdjacencyList(pub(crate) HashMap<TreeNodeType, Vec<TreeNodeType>>);

impl AdjacencyList {
    pub fn new() -> AdjacencyList {
        AdjacencyList(HashMap::new())
    }

    pub fn add(&mut self, node1: TreeNodeType, node2: TreeNodeType) {
        self.0.entry(node1).or_insert(Vec::new()).push(node2);
        self.0.entry(node2).or_insert(Vec::new()).push(node1);
    }
}
