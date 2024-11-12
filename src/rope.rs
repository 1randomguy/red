
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RopeWeight(pub u64);

pub enum Rope {
    Node { weight: RopeWeight, left: Box<Rope>, right: Box<Rope> },
    Leaf { weight: RopeWeight, content: String },
}

pub struct RopeLike {
    left: Box<Option<RopeLike>>,
    right: Box<Option<RopeLike>>,
    weight: RopeWeight,
    content: Option<String>,
}

impl Rope {
    fn collect_leaves_rec(&self) -> String {
        match self {
            Rope::Node { left, right, .. } => left.collect_leaves_rec() + &right.collect_leaves_rec(),
            Rope::Leaf { content, .. } => content.to_owned(),
        }
    }
    fn collect_leaves(&self) -> Vec<Rope> {
        let mut stack = Vec::new();
        let mut leaves = Vec::new();
        let c = self;
        while true {
            stack.push(c);
            let c = 
        }
        while !stack.is_empty() {

        }
    }
}
