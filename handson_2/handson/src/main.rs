#[derive(Debug)]
struct SegmentTree {
    size: u32,
    tree: Vec<u32>,
}

impl SegmentTree {
    fn new(size: u32) -> Self {
        Self {
            size,
            tree: vec![0; 2 * size as usize],
        }
    }

    fn print_tree(&self) {
        println!("{:?}", self.tree);
    }
}
fn main() {
    let tree = SegmentTree::new(5);
    tree.print_tree();
}
