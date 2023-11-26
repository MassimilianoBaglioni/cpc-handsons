//This struct is used for bothe the assignements.
pub struct SegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
    size: usize,
}

//To instantiate a new segment tree, call new with the array size first and then call the build funzion on the array.
impl SegmentTree {
    pub fn new(size: usize) -> Self {
        let tree = vec![i64::MIN; size.next_power_of_two() * 2 - 1];
        let lazy = vec![i64::MAX; size.next_power_of_two() * 2 - 1];
        Self { tree, lazy, size }
    }

    //Wrapper function to call the build routin with just one parameter.
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

    //Wrapper function for query_recursive.
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

    //Wrapepr function for update_range_recursive function.
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
            // Propagate changes.
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

    //The below function propagates changes using the current node and the lazy tree.
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

    //Wrapper function for query_vectors_recursive, that returns 1 or 0 instead of bool.
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
            //Current node has the value we are searching so it is in the range, we are not going any further.
            if self.tree[node] == value {
                return true;
            }
            //The value we are searching is greater than the max of the range so the value cannot be in this range, we are not going any further then.
            if self.tree[node] < value {
                return false;
            }
        }

        //We are in a leaf and we didn't return before so the value is not here for sure.
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

//This function takes the list of segments and returns a vector that stores the number of segments at each point.
pub fn sweep(array: &mut Vec<Vec<i64>>) -> Vec<i64> {
    let mut max = -1;
    //Array that stores [start, +1] and [end, -1] for each vector, the size is of course 2n.
    let mut events = vec![[0, 0]; 2 * array.len()];

    array.sort_by(|a, b| a.cmp(b));

    //Fill events array.
    for (index, el) in array.iter().enumerate() {
        events[index * 2][0] = el[0];
        events[index * 2][1] = 1;
        events[index * 2 + 1][0] = el[1] + 1;
        events[index * 2 + 1][1] = -1;
        if el[1] > max {
            max = el[1];
        }
    }

    //The result array of the routine will be long as the rightmost point occopied by a segment. With this length we are able to store every point covered by segments.
    let mut result_array = vec![0; max as usize + 2];

    events.sort_by(|a, b| a.cmp(b));

    let mut events_pointer = 0;
    let mut crt = 0;

    //By using the two pointers technique we are going trough both arrays so we are spending n+m time complexity. It's not n^2 since in the while loop we are iterating only trough the events len one time for the whole loop and not for each cycle.
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
