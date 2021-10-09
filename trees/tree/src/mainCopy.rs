use self::BinaryTree::*;

#[derive(Debug)]
enum BinaryTree<T> {
  Empty,
  NonEmpty(Box<TreeNode<T>>)
}
#[derive(Debug)]
struct TreeNode<T> {
  element: T,
  left: BinaryTree<T>,
  right: BinaryTree<T>
}

// The state of an in-order traversal of a `BinaryTree`.
struct TreeIter<'a, T: 'a> {
    // A stack of references to tree nodes. Since we use `Vec`'s
    // `push` and `pop` methods, the top of the stack is the end of the
    // vector.
    //
    // The node the iterator will visit next is at the top of the stack,
    // with those ancestors still unvisited below it. If the stack is empty,
    // the iteration is over.
    unvisited: Vec<&'a TreeNode<T>>
}

impl<'a, T: 'a> TreeIter<'a, T> {
  fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
      while let NonEmpty(ref node) = *tree {
          self.unvisited.push(node);
          tree = &node.left;
      }
  }
}

impl<T> BinaryTree<T> {
  fn iter(&self) -> TreeIter<T> {
      let mut iter = TreeIter { unvisited: Vec::new() };
      iter.push_left_edge(self);
      iter
  }

  // fn add(&mut self, value: T) {
  //   match *self {
  //       BinaryTree::Empty =>
  //           *self = BinaryTree::NonEmpty(Box::new(TreeNode {
  //               element: value,
  //               left: BinaryTree::Empty,
  //               right: BinaryTree::Empty,
  //           })),
  //       BinaryTree::NonEmpty(ref mut node) =>
  //           if value <= node.element {
  //               node.left.add(value);
  //           } else {
  //               node.right.add(value);
  //           }
  //   }
  // }

}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
  type Item = &'a T;
  type IntoIter = TreeIter<'a, T>;
  fn into_iter(self) -> Self::IntoIter {
      self.iter()
  }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<&'a T> {
      // Find the node this iteration must produce,
      // or finish the iteration.
      let node = match self.unvisited.pop() {
          None => return None,
          Some(n) => n
      };
      // The next node after this one is the leftmost child of
      // this node's right child, so push the path from here down.
      self.push_left_edge(&node.right);
      // Produce a reference to this node's value.
      Some(&node.element)
  }
}

fn make_node<T>(left: BinaryTree<T>, element: T, right: BinaryTree<T>)
           -> BinaryTree<T>
{
    NonEmpty(Box::new(TreeNode { left, element, right }))
}

fn main() {
  // Build a small tree.
  let subtree_l = make_node(Empty, 1, Empty);
  let subtree_rl = make_node(Empty, 4, Empty);
  let subtree_r = make_node(subtree_rl, 3, Empty);
  let tree = make_node(subtree_l, 12, subtree_r);
  // println!("{:#?}", &tree);

  // Iterate over it.
  let mut v = Vec::new();
  for kind in &tree {
      v.push(*kind);
  }

  println!("{:#?}", &v);
  // assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);
}