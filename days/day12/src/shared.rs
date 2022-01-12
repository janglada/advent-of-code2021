use itertools::Itertools;
use itertools::{cloned, concat};
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy)]
pub enum TreeNodeType {
    Start,
    End,
    Big(&'static str),
    Small(&'static str),
}

impl fmt::Display for TreeNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TreeNodeType::Start => write!(f, "start"),
            TreeNodeType::End => write!(f, "end"),
            TreeNodeType::Big(c) => write!(f, "{}", c),
            TreeNodeType::Small(c) => write!(f, "{}", c),
        }
    }
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

    pub fn get_start(&self) -> &Vec<TreeNodeType> {
        self.0.get(&TreeNodeType::Start).unwrap()
    }

    // pub fn map_until(&self, paths: &mut Vec<Vec<TreeNodeType>>) -> Option<Vec<Vec<TreeNodeType>>> {
    //     let p = paths
    //         .iter()
    //         .flat_map(|&v| {
    //             let leaf = v.last().unwrap();
    //             match leaf {
    //                 TreeNodeType::End => v.iter().map(|x| x),
    //                 _ => self
    //                     .0
    //                     .get(v.last().unwrap()) // children of last node in tree
    //                     .unwrap()
    //                     .into_iter()
    //                     // avoid "c"repetitions of
    //                     .filter(|&c| match c {
    //                         TreeNodeType::Small(_) => {
    //                             v.clone().into_iter().filter(|n| *n == *c).count() == 0
    //                         }
    //                         _ => true,
    //                     })
    //                     .map(move |&c| concat(vec![v.clone(), vec![c]])),
    //             }
    //         })
    //         .collect();
    //
    //     None
    // }

    pub fn map_until2(&self, paths: &mut Vec<Vec<TreeNodeType>>) -> Option<Vec<Vec<TreeNodeType>>> {
        let mut paths_next: Vec<Vec<TreeNodeType>> = vec![];
        for v in paths.iter() {
            // println!("VEC {:?}", v);
            match v.last() {
                None => return None,
                Some(leaf) => {
                    match leaf {
                        TreeNodeType::End => {
                            paths_next.push(v.clone());
                        }
                        _ => {
                            self.0
                                .get(leaf) // children of last node in tree
                                .unwrap()
                                .into_iter()
                                // avoid "c"repetitions of
                                .filter(|&c| match c {
                                    TreeNodeType::Small(_) => {
                                        v.clone().into_iter().filter(|n| *n == *c).count() == 0
                                    }
                                    TreeNodeType::Start => false,
                                    _ => true,
                                })
                                .for_each(|&c| {
                                    paths_next.push(concat(vec![v.clone(), vec![c]]));
                                })
                        }
                    };
                }
            }
        }
        return Some(paths_next);
        // let p = paths
        //     .iter()
        //     .flat_map(|&v| {
        //         let leaf = v.last().unwrap();
        //         match leaf {
        //             TreeNodeType::End => v.iter().map(|x| x),
        //             _ => self
        //                 .0
        //                 .get(v.last().unwrap()) // children of last node in tree
        //                 .unwrap()
        //                 .into_iter()
        //                 // avoid "c"repetitions of
        //                 .filter(|&c| match c {
        //                     TreeNodeType::Small(_) => {
        //                         v.clone().into_iter().filter(|n| *n == *c).count() == 0
        //                     }
        //                     _ => true,
        //                 })
        //                 .map(move |&c| concat(vec![v.clone(), vec![c]])),
        //         }
        //     })
        //     .collect();

        None
    }
}
