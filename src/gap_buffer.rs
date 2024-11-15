#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferPos(u64);

struct GapBuffer {
}

impl GapBuffer {
    fn insert_text(&self, cursor: u64, text: u64) {};
}
