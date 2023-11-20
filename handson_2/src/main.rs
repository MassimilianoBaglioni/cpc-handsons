#[derive(Debug)]
struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(array: &[i32]) -> Self {
        let mut tree = vec![0; 2 * array.len()];
        tree[array.len()..].copy_from_slice(array);

        for i in (1..array.len()).rev() {
            tree[i] = tree[i * 2].max(tree[i * 2 + 1]);
        }

        SegmentTree {
            size: array.len(),
            tree,
        }
    }

    fn print_tree(&self) {
        println!("{:?}", self.tree);
    }

    fn update(&self, i: usize, j: usize, t: usize) -> Vec<u32> {
        println!("called update on {},{},{}", i, j, t);
        vec![0; 2]
    }
    fn query_max(&self, left: usize, right: usize) -> i32 {
        let mut left = left + self.size - 1;
        let mut right = right + self.size - 1;
        let mut max_val = std::i32::MIN;

        while left <= right {
            if left % 2 == 1 {
                max_val = max_val.max(self.tree[left]);
                left += 1;
            }
            if right % 2 == 0 {
                max_val = max_val.max(self.tree[right]);
                right -= 1;
            }
            left /= 2;
            right /= 2;
        }

        max_val
    }

    fn update_max_iterative(&mut self, i: usize, j: usize, t: i32) {
        let mut i = i + self.size;
        let mut j = j + self.size;
        while i <= j {
            if i % 2 == 1 {
                self.tree[i] = self.tree[i].min(t);
                i += 1;
            }
            if j % 2 == 0 {
                self.tree[j] = self.tree[j].min(t);
                j -= 1;
            }
            j /= 2;
            j /= 2;
        }
    }

    //TODO j is exclusive.
    fn max(&self, mut i: usize, mut j: usize) -> i32 {
        i += self.size;
        j += self.size;
        println!(" first init i:{}j:{}", i, j);

        let mut max_val = std::i32::MIN;
        println!("tree: {:?}", self.tree);

        while i < j {
            if (i & 1) == 1 {
                //i is odd
                max_val = max_val.max(self.tree[i]);
                println!("i is odd i:{}j:{} new max assigned {}", i, j, max_val);
                i += 1;
            }
            if (j & 1) == 1 {
                j -= 1;
                max_val = max_val.max(self.tree[j]);
                println!("j is odd i:{}j:{} new max assigned {}", i, j, max_val);
            }
            println!("pre / i:{}j:{}", i, j);

            i /= 2;
            j /= 2;
            println!("i:{}j:{}", i, j);
        }
        //println!("called max on {},{},", i, j);
        max_val
    }
}

#[cfg(test)]
mod tests {}

fn run_tests() {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    let directory_path = "src/Testset_handson2_2324_p1/";
    let mut tree = SegmentTree::new(&[1, 2, 3, 4]);

    //TODO Hardcoded number of txt should refactor and count the number of txt files in the folder
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
            .filter(|&num| num != '\n' as i32) // Exclude newline characters
            .collect();

        let mut all_input_values: Vec<Vec<i32>> = Vec::new();

        for line in input_lines.iter() {
            let input_values: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            all_input_values.push(input_values);
        }

        //TODO must update the tree for every input file with the given array.
        //TODO implement asserts here.
        let input_values = &all_input_values[1];
        let n = &all_input_values[0][0];
        let mut output_index = 0;
        tree = SegmentTree::new(&input_values);
        println!("working on input: {}", input_filename);

        for line in all_input_values.iter().skip(2) {
            if line[0] == 0 {
                tree.update(line[1] as usize, line[2] as usize, line[3] as usize);
            } else if line[0] == 1 {
                println!("assert {:?}", output_array);
                println!(
                    "max return {}",
                    tree.query_max(line[1] as usize, line[2] as usize)
                );
                assert!(
                    tree.query_max(line[1] as usize, line[2] as usize)
                        == output_array[output_index]
                );
                output_index += 1;
            }
        }
    }
}

fn main() {
    let mut tree = SegmentTree::new(&[5, 1, 4, 3, 2]);

    //println!("{} max", tree.max(2, 3));
    //println!("{} max", tree.max(0, 2));
    //println!("{} new max", tree.query_max(5, 5));
    tree.update_max_iterative(1, 2, 2);
    println!("{:?}", tree.tree);
    //run_tests();
}
