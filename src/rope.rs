mod text_buffer;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RopeWeight(pub u64);

enum Node {
    BranchNode { weight: RopeWeight, left: Box<Node>, right: Box<Node> },
    LeafNode(String),
}

pub struct Rope {
    root: Node,
}



impl Rope {
    // fn collect_leaves_rec(&self) -> String {
    //     match self.root {
    //         Node::BranchNode { left, right, .. } => left.collect_leaves_rec() + &right.collect_leaves_rec(),
    //         Node::LeafNode { content, .. } => content.to_owned(),
    //     }
    // }
    fn debug_print(&self) -> String {
        //
        "Not inplemented yet".to_string()
    }
    fn collect_leaves(&self) -> Vec<Node> {
        let mut stack = Vec::new();
        let mut leaves = Vec::new();
        let current_node = self.root;
        loop {
            stack.push(current_node);
            let c = ;
        }
        while !stack.is_empty() {

        }
    }
    fn insert_text(&self, cursor: u64, text: String) {
        // split
        // append_node
        // concat
        // rebalance
    }
    fn delete_text(&self, cursor: u64, count: u64) {
        // middle = split to cursor
        // right = split middle to count
        // concat rope + right
        // delete middle
        // rebalance
    }
    fn concat(&self, right: Node) {
        // get old root weight
        // new root node
        // set new root weight, left, right
        // set new root as root
    }
    fn split(&self, cursor: u64) -> Rope {
        // complicated hehehe
    }
}
impl Node {
    fn rotate_left() {}
    fn rotate_right() {}
    fn get_weight() {}
    fn rebalance() {} //rebalance child first
}
impl Node::BranchNode {
    fn set_left() {}
}

