#[derive(Debug)]
struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
    marked: Vec<i32>,
}

impl SegmentTree {
    fn new(array: &[i32]) -> Self {
        let tree = vec![0; 4 * array.len()];
        let marked = vec![0; 4 * array.len()];
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

    fn reset_marked(&mut self) {
        self.marked = vec![0; 4 * self.size];
    }

    fn max_query(&mut self, query_l: usize, query_r: usize) -> i32 {
        self.max_query_rec(1, 0, self.size - 1, query_l - 1, query_r - 1)
    }

    //TODO this is 0 indexed and should be tested.
    fn max_query_rec(
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
        if query_l <= seg_l && query_r >= seg_r {
            return self.tree[v];
        }
        self.push(v);
        let mid = (seg_l + seg_r) / 2;
        return self
            .max_query_rec(v * 2, seg_l, mid, query_l, query_r.min(mid))
            .max(self.max_query_rec(v * 2 + 1, mid + 1, seg_r, query_l.max(mid + 1), query_r));
    }

    fn push(&mut self, v: usize) {
        if self.marked[v] != 0 {
            self.tree[v * 2] = self.tree[v].max(self.tree[v * 2]);
            self.tree[v * 2 + 1] = self.tree[v].max(self.tree[v * 2 + 1]);
            self.marked[v * 2] = self.marked[v];
            self.marked[v * 2 + 1] = self.marked[v];
            self.marked[v] = 0;
        }
    }

    fn update(&mut self, l: usize, r: usize, new_val: i32) {
        self.update_rec(1, 0, self.size - 1, l - 1, r - 1, new_val);
    }

    fn update_rec(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize, new_val: i32) {
        if l > r {
            return;
        }
        if l == tl && tr == r {
            if self.tree[v].min(new_val) == new_val {
                self.tree[v] = self.tree[v].min(new_val);
                self.marked[v] = new_val;
            }
        } else {
            self.push(v);
            let mid = (tl + tr) / 2;
            self.update_rec(v * 2, tl, mid, l, r.min(mid), new_val);
            self.update_rec(v * 2 + 1, mid + 1, tr, r.max(mid + 1), r, new_val);
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
        let mut tree = SegmentTree::new(&[5, 1, 4, 3, 2]);

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
            let output_array: Vec<i32> = output_contents
                .lines()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            println!("output parsed {:?}", output_array);

            let mut all_input_values: Vec<Vec<i32>> = Vec::new();

            for line in input_lines.iter() {
                let input_values: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                all_input_values.push(input_values);
            }

            let input_values = &all_input_values[1];
            let n = &all_input_values[0][0];
            let mut output_index = 0;
            tree = SegmentTree::new(&input_values);
            tree.build_tree(&input_values, 1, 0, tree.size - 1);
            tree.reset_marked();
            println!("working on input: {}", input_filename);

            for line in all_input_values.iter().skip(2) {
                if line[0] == 0 {
                    println!("update {:?}", line);
                    tree.update(line[1] as usize, line[2] as usize, line[3]);
                } else if line[0] == 1 {
                    println!(
                        "assert {:?} and return {}",
                        output_array[output_index],
                        tree.max_query(line[1] as usize, line[2] as usize)
                    );
                    assert!(
                        tree.max_query(line[1] as usize, line[2] as usize)
                            == output_array[output_index]
                    );
                    output_index += 1;
                }
            }
        }
    }
}
fn main() {
    let mut tree = SegmentTree::new(&[9, 4, 1, 6, 5, 10, 6, 8, 7, 4]);
    tree.build_tree(&[9, 4, 1, 6, 5, 10, 6, 8, 7, 4], 1, 0, tree.size - 1);
    //println!("Basic tree: {:?}", tree.tree);
    //tree.update_rec(1, 0, tree.size - 1, 0, 1, 2);
    tree.update(6, 7, 10);
    tree.update(3, 10, 4);

    println!("{}", tree.max_query(2, 4));
    //println!("update_recd tree: {:?}", tree.tree);
    //println!("tree post update_rec {:?}", tree.marked);
    //println!("RESULT {}", tree.max_query_rec(1, 0, tree.size - 1, 1, 3));
    //println!("tree post update_rec {:?}", tree.tree);
}
