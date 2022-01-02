use itertools::Itertools;

#[derive(PartialEq, Clone)]
pub enum TreeNodeType {
    Start,
    End,
    Big(String),
    Small(String),
}

impl TreeNodeType {
    pub fn from_text(s: &str) -> TreeNodeType {
        if s == "start" {
            TreeNodeType::Start
        } else if s == "end" {
            TreeNodeType::End
        } else if s.eq(s.to_uppercase().as_str()) {
            TreeNodeType::Big(s.to_string())
        } else {
            TreeNodeType::Small(s.to_string())
        }
    }
}

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct CavePath {
    nodes: Vec<NodeIndex>,
}

pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
    pub node_type: TreeNodeType,
}
pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

impl CavePath {
    pub fn add_node(&mut self, node: NodeIndex) {
        self.nodes.push(node)
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn edge_at(&self, idx: NodeIndex) -> Option<&EdgeData> {
        self.edges.get(idx)
    }

    pub fn node_at(&self, idx: NodeIndex) -> Option<&NodeData> {
        self.nodes.get(idx)
    }

    pub fn find_node_by_name(&self, name: &str) -> Option<NodeIndex> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(idx, &p)| match p.node_type {
                TreeNodeType::Start => false,
                TreeNodeType::End => false,
                TreeNodeType::Big(n) => n.eq(name),
                TreeNodeType::Small(n) => n.eq(name),
            })
            .map(|x| x.0)
    }

    pub fn find_node_by_type(&self, node_type: TreeNodeType) -> Option<NodeIndex> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(idx, &p)| p.node_type == node_type)
            .map(|x| x.0)
    }

    pub fn add_node(&mut self, node_type: TreeNodeType) -> NodeIndex {
        match self.find_node_by_type(node_type) {
            None => {
                let index = self.nodes.len();
                self.nodes.push(NodeData {
                    first_outgoing_edge: None,
                    node_type,
                });
                index
            }
            Some(c) => c,
        }
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }

    pub fn from_text(input: String) -> Graph {
        let mut graph = Graph::new();
        input.lines().for_each(|l| {
            let mut iter = l.split("-");
            let n1 = iter.next().unwrap();
            let n2 = iter.next().unwrap();

            let n1_index = graph.add_node(TreeNodeType::from_text(n1));
            let n2_index = graph.add_node(TreeNodeType::from_text(n2));

            graph.add_edge(n1_index, n2_index);
        });
        graph
    }
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}
