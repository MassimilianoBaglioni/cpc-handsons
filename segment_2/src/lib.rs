pub struct SegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
    size: usize,
}

impl SegmentTree {
    pub fn new(size: usize) -> Self {
        let tree = vec![i64::MIN; size.next_power_of_two() * 2 - 1];
        let lazy = vec![i64::MAX; size.next_power_of_two() * 2 - 1];
        Self { tree, lazy, size }
    }

    pub fn build(&mut self, arr: &[i64]) {
        self.build_recursive(arr, 0, 0, arr.len() - 1);
    }

    fn build_recursive(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            self.build_recursive(arr, 2 * node + 1, start, mid);
            self.build_recursive(arr, 2 * node + 2, mid + 1, end);
            self.tree[node] = self.tree[2 * node + 1].max(self.tree[2 * node + 2]);
        }
    }

    pub fn query_range(&mut self, left: usize, right: usize) -> i64 {
        self.query_recursive(0, 0, self.size - 1, left - 1, right - 1)
    }

    fn query_recursive(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
    ) -> i64 {
        self.propagate(node, start, end);

        if right < start || left > end {
            return i64::min_value(); // Out of range
        }

        if left <= start && right >= end {
            return self.tree[node];
        }

        let mid = (start + end) / 2;
        let left_child = self.query_recursive(2 * node + 1, start, mid, left, right);
        let right_child = self.query_recursive(2 * node + 2, mid + 1, end, left, right);
        left_child.max(right_child)
    }

    pub fn update_range_with_value(&mut self, left: usize, right: usize, value: i64) {
        self.update_range_recursive(0, 0, self.size - 1, left - 1, right - 1, value);
    }

    fn update_range_recursive(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
        value: i64,
    ) {
        self.propagate(node, start, end);

        if right < start || left > end {
            return; // Out of range
        }

        if left <= start && right >= end {
            // Update the range and mark for lazy propagation
            self.tree[node] = self.tree[node].min(value);
            if start != end {
                self.lazy[2 * node + 1] = self.lazy[2 * node + 1].min(value);
                self.lazy[2 * node + 2] = self.lazy[2 * node + 2].min(value);
            }
            return;
        }

        let mid = (start + end) / 2;
        self.update_range_recursive(2 * node + 1, start, mid, left, right, value);
        self.update_range_recursive(2 * node + 2, mid + 1, end, left, right, value);

        self.tree[node] = self.tree[2 * node + 1].max(self.tree[2 * node + 2]);
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] != i64::MAX {
            // Update the node and mark for lazy propagation
            self.tree[node] = self.tree[node].min(self.lazy[node]);
            if start != end {
                self.lazy[2 * node + 1] = self.lazy[node].min(self.lazy[2 * node + 1]);
                self.lazy[2 * node + 2] = self.lazy[node].min(self.lazy[2 * node + 2]);
            }
            self.lazy[node] = i64::MAX; // Reset lazy value
        }
    }

    pub fn query_vectors(&mut self, left: usize, right: usize, value: i64) -> i32 {
        if self.query_vectors_recursive(0, 0, self.size - 1, left, right, value) {
            1
        } else {
            0
        }
    }

    fn query_vectors_recursive(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
        value: i64,
    ) -> bool {
        if right < start || left > end {
            return false; // Out of range
        }

        if left <= start && right >= end {
            if self.tree[node] == value {
                return true;
            }
            if self.tree[node] < value {
                return false;
            }
        }

        if start == end {
            return false;
        }

        let mid = (start + end) / 2;
        let left_child = self.query_vectors_recursive(2 * node + 1, start, mid, left, right, value);
        let right_child =
            self.query_vectors_recursive(2 * node + 2, mid + 1, end, left, right, value);
        left_child || right_child
    }
}

//Overall complexity n log n
pub fn sweep(array: &mut Vec<Vec<i64>>) -> Vec<i64> {
    let mut max = -1;
    let mut events = vec![[0, 0]; 2 * array.len()];

    //Using n log n to sort.
    array.sort_by(|a, b| a.cmp(b));
    for (index, el) in array.iter().enumerate() {
        events[index * 2][0] = el[0];
        events[index * 2][1] = 1;
        events[index * 2 + 1][0] = el[1] + 1;
        events[index * 2 + 1][1] = -1;
        if el[1] > max {
            max = el[1];
        }
    }
    let mut result_array = vec![0; max as usize + 2];

    events.sort_by(|a, b| a.cmp(b));

    let mut events_pointer = 0;
    let mut crt = 0;
    //Using double pointer technique we are going trough both arrays so we are spending n+m time complexity.
    for i in 0..result_array.len() {
        while events[events_pointer][0] == i as i64 {
            crt += events[events_pointer][1];
            result_array[i] = crt;
            events_pointer += 1;
            if events_pointer == events.len() {
                break;
            }
        }
        result_array[i] = crt;
    }

    return result_array[0..result_array.len() - 1].to_vec();
}
