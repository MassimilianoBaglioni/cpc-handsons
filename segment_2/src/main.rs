struct SegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
    size: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut tree = vec![i64::MIN; 4 * size];
        let lazy = vec![i64::MAX; 4 * size];
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

    fn query_vectors(&mut self, left: usize, right: usize, value: i64) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    #[test]
    fn run_tests() {
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

    #[test]
    fn test2() {
        let directory_path = "src/Testset_handson2_2324_p2/";

        for i in 0..=7 {
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

            let mut all_input_values: Vec<Vec<i64>> = Vec::new();

            for line in input_lines.iter() {
                let input_values: Vec<i64> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();

                all_input_values.push(input_values);
            }

            let mut input_vectors: Vec<Vec<i64>> = Vec::new();
            for i in 1..=all_input_values[0][0] {
                input_vectors.push(all_input_values[i as usize].clone());
            }

            let leaves_array = sweep(&mut input_vectors);
            let mut tree = SegmentTree::new(leaves_array.len());
            tree.build(&leaves_array);

            for (index, query) in (all_input_values[0][0] + 1
                ..=all_input_values[0][0] + all_input_values[0][1])
                .enumerate()
            {
                println!(
                    "returned: {} expected: {}",
                    tree.query_vectors(
                        all_input_values[query as usize][0] as usize,
                        all_input_values[query as usize][1] as usize,
                        all_input_values[query as usize][2]
                    ),
                    output_array[index as usize] as i32
                );

                assert!(
                    tree.query_vectors(
                        all_input_values[query as usize][0] as usize,
                        all_input_values[query as usize][1] as usize,
                        all_input_values[query as usize][2]
                    ) == output_array[index as usize] as i32
                );
            }
        }
    }
}

//Overall complexity n log n
fn sweep(array: &mut Vec<Vec<i64>>) -> Vec<i64> {
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

fn main() {
    let mut vector_of_vectors: Vec<Vec<i64>> = vec![
        vec![2, 6],
        vec![3, 8],
        vec![4, 6],
        vec![1, 1],
        vec![5, 9],
        vec![6, 7],
        vec![8, 9],
        vec![0, 7],
        vec![1, 2],
        vec![2, 7],
    ];
    let leaves = sweep(&mut vector_of_vectors);
    //println!("{:?}", leaves);
    let mut tree = SegmentTree::new(leaves.len());
    tree.build(&leaves);
}
