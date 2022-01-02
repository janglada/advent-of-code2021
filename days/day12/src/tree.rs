use crate::shared::TreeNodeType;
use trees::{Node, Tree};

// pub struct TreeWrapper<'a> {
//     tree: &'a Tree<TreeNodeType>,
// }
//
// pub struct TreeWrapperIntoIterator<'a> {
//     stack: Vec<&'a TreeWrapper<'a>>,
// }
//
// // impl<'a> LinearContentIterator<'a> {
// //     fn new(root: &'a mut Foo) -> LinearContentIterator<'a> {
// //         let mut stack = Vec::with_capacity(100);
// //         stack.push(root);
// //         LinearContentIterator { stack }
// //     }
// // }
// // https://users.rust-lang.org/t/mutable-iterator-for-tree-like-struct/28050
// impl<'a> IntoIterator for TreeWrapper<'a> {
//     type Item = TreeNodeType;
//     type IntoIter = TreeWrapperIntoIterator<'a>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         let mut stack = Vec::with_capacity(100);
//         stack.push(&self);
//         TreeWrapperIntoIterator { stack }
//     }
// }
//
// impl<'a> Iterator for TreeWrapperIntoIterator<'a> {
//     type Item = TreeNodeType;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.stack.pop() {
//             None => None,
//             Some(node) => match node.tree.parent() {
//                 None => None,
//                 Some(parent) => {
//                     self.stack.push(TreeWrapper { tree: parent });
//                     Some(*node.tree.data())
//                 }
//             },
//         }
//     }
// }
