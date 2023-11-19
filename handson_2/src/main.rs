#[derive(Debug)]
struct SegmentTree {
    size: u32,
    tree: Vec<u32>,
}

impl SegmentTree {
    fn new(size: u32) -> Self {
        let tree = vec![0; 2 * size as usize];

        SegmentTree { size, tree }
    }

    fn init_tree() {}

    fn print_tree(&self) {
        println!("{:?}", self.tree);
    }

    fn update(&self, i: i32, j: i32, t: i32) -> Vec<u32> {
        println!("called update on {},{},{}", i, j, t);
        vec![0; 2]
    }

    fn max(&self, i: i32, j: i32) -> i32 {
        println!("called max on {},{},", i, j);
        32
    }
}

#[cfg(test)]
mod tests {}

fn run_tests() {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    let directory_path = "src/Testset_handson2_2324_p1/";
    let tree = SegmentTree::new(5);

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

        for line in all_input_values.iter().skip(2) {
            if line[0] == 0 {
                tree.update(line[1], line[2], line[3]);
            } else if line[0] == 1 {
                tree.max(line[1], line[2]);
                println!("assert {}", output_array[output_index]);
                output_index += 1;
            }
        }
    }
}

fn main() {
    let tree = SegmentTree::new(5);
    tree.print_tree();
    run_tests()
}
