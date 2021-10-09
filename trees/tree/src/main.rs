use rand::Rng;

/*
Binary Tree implementaion courtesy of
Programming Rust: Fast, Safe Systems Development
By Jim Blandy, Jason Orendorff
*/
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

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty =>
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })),
            BinaryTree::NonEmpty(ref mut node) =>
                match node.left {
                    // buggy code
                    // tree leans hard right i.e. skips over
                    // left branches
                    // proper iterator is needed
                    BinaryTree::Empty => node.left.add(value),
                    BinaryTree::NonEmpty(_) => node.right.add(value)
                }
        }
    }

    /// TODO:
    // iterateViaBFS()

    // printToConsole()

    // writeToFile()

    // readFromFile()
}

fn main() {
    let mut tree = BinaryTree::Empty;
    let num = rand::thread_rng().gen_range(0..100);
    for i in 1..10 {
        tree.add(i * num as i32);
    }
    println!("{:#?}", &tree);
}