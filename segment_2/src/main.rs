struct SegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
    size: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut tree = vec![0; 4 * size];
        let lazy = vec![0; 4 * size];
        Self { tree, lazy, size }
    }

    fn build(&mut self, arr: &[i64]) {
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

    fn query_range(&mut self, left: usize, right: usize) -> i64 {
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

    fn update_range_with_value(&mut self, left: usize, right: usize, value: i64) {
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
                self.lazy[2 * node + 1] = value;
                self.lazy[2 * node + 2] = value;
            }
            return;
        }

        let mid = (start + end) / 2;
        self.update_range_recursive(2 * node + 1, start, mid, left, right, value);
        self.update_range_recursive(2 * node + 2, mid + 1, end, left, right, value);

        self.tree[node] = self.tree[2 * node + 1].max(self.tree[2 * node + 2]);
    }

    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] != 0 {
            // Update the node and mark for lazy propagation
            self.tree[node] = self.tree[node].min(self.lazy[node]);
            if start != end {
                self.lazy[2 * node + 1] = self.lazy[node];
                self.lazy[2 * node + 2] = self.lazy[node];
            }
            self.lazy[node] = 0; // Reset lazy value
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn run_tests() {
        use super::*;
        use std::fs;
        use std::path::Path;
        use std::path::PathBuf;

        let directory_path = "src/Testset_handson2_2324_p1/";
        let mut tree: SegmentTree;

        for i in 0..=10 {
            let input_filename = format! {"input{}.txt", i};
            let output_filename = format! {"output{}.txt", i};

            let input_full_path = PathBuf::from(directory_path).join(Path::new(&input_filename));
            let output_full_path = PathBuf::from(directory_path).join(Path::new(&output_filename));

            let input_contents =
                fs::read_to_string(input_full_path).expect("Failed to open the test file.");
            let output_contents =
                fs::read_to_string(output_full_path).expect("Failed to open the test file.");

            let input_lines: Vec<&str> = input_contents.lines().collect();
            let output_array: Vec<i64> = output_contents
                .lines()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            println!("output parsed {:?}", output_array);

            let mut all_input_values: Vec<Vec<i64>> = Vec::new();

            for line in input_lines.iter() {
                let input_values: Vec<i64> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();

                all_input_values.push(input_values);
            }

            let input_values = &all_input_values[1];
            let mut output_index = 0;
            tree = SegmentTree::new(input_values.len());
            tree.build(input_values);
            println!("working on input: {}", input_filename);

            for line in all_input_values.iter().skip(2) {
                if line[0] == 0 {
                    println!("update {:?}", line);
                    tree.update_range_with_value(line[1] as usize, line[2] as usize, line[3]);
                } else if line[0] == 1 {
                    println!(
                        "{:?} my:{}, first value {}, second value {}",
                        output_array[output_index],
                        tree.query_range(line[1] as usize, line[2] as usize),
                        line[1],
                        line[2],
                    );
                    assert!(
                        tree.query_range(line[1] as usize, line[2] as usize)
                            == output_array[output_index]
                    );
                    output_index += 1;
                }
            }
        }
    }
}

fn main() {
    let arr = vec![9, 4, 1, 6, 5, 10, 6, 8, 7, 4];
    let mut segment_tree = SegmentTree::new(arr.len());
    segment_tree.build(&arr);

    // Test 1
    println!("Initial array: {:?}", arr);
    println!("Max in range [5, 8]: {}", segment_tree.query_range(5, 8));

    // Test 2
    segment_tree.update_range_with_value(6, 7, 10);
    println!("Max in range [6, 8]: {}", segment_tree.query_range(6, 8));

    // Test 3
    segment_tree.update_range_with_value(3, 10, 4);
    println!("Max in range [2, 4]: {}", segment_tree.query_range(2, 4));

    // Test 4
    println!("Max in range [1, 9]: {}", segment_tree.query_range(1, 9));

    // Test 5
    segment_tree.update_range_with_value(8, 9, 3);
    println!("Max in range [8, 8]: {}", segment_tree.query_range(8, 8));

    // Test 6
    segment_tree.update_range_with_value(4, 5, 7);
    println!("Max in range [4, 10]: {}", segment_tree.query_range(4, 10));

    // Test 7
    segment_tree.update_range_with_value(9, 10, 1);
    println!(
        "Max in range [10, 10]: {}",
        segment_tree.query_range(10, 10)
    );

    // Test 8
    segment_tree.update_range_with_value(1, 3, 2);
    println!("Max in range [1, 3]: {}", segment_tree.query_range(1, 3));

    // Test 9
    println!("Max in range [7, 9]: {}", segment_tree.query_range(7, 9));

    // Test 10
    segment_tree.update_range_with_value(2, 6, 5);
    println!("Max in range [2, 6]: {}", segment_tree.query_range(2, 6));

    // Additional Tests
    segment_tree.update_range_with_value(4, 6, 15);
    println!("Max in range [4, 6]: {}", segment_tree.query_range(4, 6));

    segment_tree.update_range_with_value(1, 5, 8);
    println!("Max in range [1, 5]: {}", segment_tree.query_range(1, 5));

    segment_tree.update_range_with_value(3, 8, 12);
    println!("Max in range [3, 8]: {}", segment_tree.query_range(3, 8));

    segment_tree.update_range_with_value(1, 10, 20);
    println!("Max in range [1, 10]: {}", segment_tree.query_range(1, 10));

    segment_tree.update_range_with_value(5, 9, 3);
    println!("Max in range [5, 9]: {}", segment_tree.query_range(5, 9));

    segment_tree.update_range_with_value(2, 7, 10);
    println!("Max in range [2, 7]: {}", segment_tree.query_range(2, 7));

    segment_tree.update_range_with_value(1, 4, 5);
    println!("Max in range [1, 4]: {}", segment_tree.query_range(1, 4));

    segment_tree.update_range_with_value(6, 8, 18);
    println!("Max in range [6, 8]: {}", segment_tree.query_range(6, 8));

    segment_tree.update_range_with_value(3, 5, 7);
    println!("Max in range [3, 5]: {}", segment_tree.query_range(3, 5));

    segment_tree.update_range_with_value(1, 7, 14);
    println!("Max in range [1, 7]: {}", segment_tree.query_range(1, 7));

    segment_tree.update_range_with_value(4, 8, 22);
    println!("Max in range [4, 8]: {}", segment_tree.query_range(4, 8));

    segment_tree.update_range_with_value(2, 9, 13);
    println!("Max in range [2, 9]: {}", segment_tree.query_range(2, 9));

    segment_tree.update_range_with_value(1, 6, 25);
    segment_tree.update_range_with_value(1, 6, 2);
    println!("Max in range [1, 6]: {}", segment_tree.query_range(1, 6));
}
