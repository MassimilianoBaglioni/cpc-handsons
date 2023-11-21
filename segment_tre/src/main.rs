#[derive(Debug)]
struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
    marked: Vec<bool>,
}

impl SegmentTree {
    fn new(array: &[i32]) -> Self {
        let tree = vec![0; 4 * array.len()];
        let marked = vec![false; 4 * array.len()];
        SegmentTree {
            size: array.len(),
            tree,
            marked,
        }
    }

    fn build_tree(&mut self, array: &[i32], v: usize, left: usize, right: usize) {
        if left == right {
            self.tree[v] = array[left];
        } else {
            let mid = (left + right) / 2;
            self.build_tree(&array, v * 2, left, mid);
            self.build_tree(&array, v * 2 + 1, mid + 1, right);
            self.tree[v] = self.tree[v * 2].max(self.tree[v * 2 + 1]);
        }
    }

    //TODO this is 0 indexed and should be tested.
    fn max_query(
        &mut self,
        v: usize,
        seg_l: usize,
        seg_r: usize,
        query_l: usize,
        query_r: usize,
    ) -> i32 {
        if query_l > query_r {
            return i32::MIN; // Return the minimum value for an empty range
        }
        if query_l == seg_l && query_r == seg_r {
            return self.tree[v];
        }
        self.push(v);
        let mid = (seg_l + seg_r) / 2;
        return self
            .max_query(v * 2, seg_l, mid, query_l, query_r.min(mid))
            .max(self.max_query(v * 2 + 1, mid + 1, seg_r, query_l.max(mid + 1), query_r));
    }

    fn push(&mut self, v: usize) {
        if self.marked[v] {
            self.tree[v * 2] = self.tree[v];
            self.tree[v * 2 + 1] = self.tree[v];
            self.marked[v * 2] = true;
            self.marked[v * 2 + 1] = true;
            self.marked[v] = false;
        }
    }

    fn update(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize, new_val: i32) {
        if l > r {
            return;
        }

        if l == tl && tr == r {
            if self.tree[v].min(new_val) == new_val {
                self.tree[v] = self.tree[v].min(new_val);
                self.marked[v] = true;
            }
        } else {
            self.push(v);
            let mid = (tl + tr) / 2;
            self.update(v * 2, tl, mid, l, r.max(mid), new_val);
            self.update(v * 2 + 1, mid + 1, tr, r.max(mid + 1), r, new_val);
        }
    }
}

fn main() {
    let mut tree = SegmentTree::new(&[9, 10, 11, 6, 5, 10, 11, 12, 13, 4]);
    tree.build_tree(&[9, 10, 11, 6, 5, 10, 11, 12, 13, 4], 1, 0, tree.size - 1);
    println!("{}", tree.max_query(1, 0, tree.size - 1, 0, 1));
    tree.update(1, 0, tree.size - 1, 0, 4, 1);
    //println!("tree post update {:?}", tree.marked);
    println!("RESULT {}", tree.max_query(1, 0, tree.size - 1, 0, 3));
    println!("tree post update {:?}", tree.tree);
}
