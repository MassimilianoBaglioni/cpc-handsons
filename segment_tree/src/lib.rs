#[derive(Debug)]
pub struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegmentTree {
    pub fn new(array: &[i32]) -> Self {
        let tree = SegmentTree::create_tree(array);
        let lazy = vec![i32::MAX; array.len().next_power_of_two() * 2 - 1];
        SegmentTree {
            size: array.len(),
            tree,
            lazy,
        }
    }

    fn create_tree(array: &[i32]) -> Vec<i32> {
        let next_power = array.len().next_power_of_two();
        let mut tree = vec![0; next_power * 2 - 1];

        for i in 0..array.len() {
            tree[i] = i32::MIN;
        }
        SegmentTree::construct_tree(&mut tree, array, 0, array.len() - 1, 0);
        return tree;
    }

    fn construct_tree(tree: &mut [i32], input: &[i32], low: usize, high: usize, v: usize) {
        if low == high {
            tree[v] = input[low];
            return;
        }
        let mid = (high + low) / 2;
        Self::construct_tree(tree, input, low, mid, 2 * v + 1);
        Self::construct_tree(tree, input, mid + 1, high, 2 * v + 2);
        tree[v] = tree[v * 2 + 1].max(tree[v * 2 + 2]);
    }

    pub fn reset_lazy(&mut self) {
        self.lazy = vec![i32::MAX; self.size.next_power_of_two() * 2 - 1];
    }

    pub fn max_query(&mut self, qlow: usize, qhigh: usize) -> i32 {
        self.max_query_rec(qlow - 1, qhigh - 1, 0, self.size - 1, 0)
    }

    //TODO this is 0 indexed and should be tested.
    fn max_query_rec(
        &mut self,
        qlow: usize,  //qlow
        qhigh: usize, //qhigh
        low: usize,   //low
        high: usize,  //high
        v: usize,
    ) -> i32 {
        if low > high {
            return i32::MIN; // Return the minimum value for an empty range
        }
        self.push(v, low, high);

        if qlow > high || qhigh < low {
            return i32::MIN;
        }
        if qlow <= low && qhigh >= high {
            return self.tree[v];
        }
        let mid = (low + high) / 2;
        return self
            .max_query_rec(qlow, qhigh, low, mid, 2 * v + 1)
            .max(self.max_query_rec(qlow, qhigh, mid + 1, high, 2 * v + 2));
    }

    fn push(&mut self, v: usize, low: usize, high: usize) {
        if self.lazy[v] != 0 {
            self.tree[v] = self.tree[v].min(self.lazy[v]);
            if low != high {
                self.lazy[2 * v + 1] = self.lazy[v].min(self.lazy[2 * v + 1]);
                self.lazy[2 * v + 2] = self.lazy[v].min(self.lazy[2 * v + 2]);
            }
            self.lazy[v] = i32::MAX;
        }
    }

    pub fn update(&mut self, start_range: usize, end_range: usize, new_val: i32) {
        self.update_rec(start_range, end_range, new_val, 0, self.size - 1, 0);
    }

    fn update_rec(
        &mut self,

        start_range: usize,
        end_range: usize,
        new_val: i32,
        low: usize,

        high: usize,
        v: usize,
    ) {
        //low and high are
        if low > high {
            return;
        }

        self.push(v, low, high);

        if start_range > high || end_range < low {
            return;
        }

        if start_range <= low && end_range >= high {
            self.tree[v] = self.tree[v].min(new_val);
            if low != high {
                self.lazy[2 * v + 1] = self.lazy[2 * v + 1].min(new_val);
                self.lazy[2 * v + 2] = self.lazy[2 * v + 2].min(new_val);
            }
            return;
        }

        let mid = (low + high) / 2;
        self.update_rec(start_range, end_range, new_val, low, mid, 2 * v + 1);
        self.update_rec(start_range, end_range, new_val, mid + 1, high, 2 * v + 2);
        self.tree[v] = self.tree[v]
            .max(self.tree[2 * v + 1])
            .max(self.tree[2 * v + 2]);
    }
}
