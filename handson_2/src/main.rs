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
}

#[cfg(test)]
mod tests {}

fn run_tests() {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    let input_filename = "input0.txt";
    let output_filename = "output0.txt";
    let directory_path = "src/Testset_handson2_2324_p1/";

    for i in 0..=10 {
        let mut file_name = format! {"input{}.txt", i};
        println!("{}", file_name);
    }

    let input_full_path = PathBuf::from(directory_path).join(Path::new(input_filename));
    let output_full_path = PathBuf::from(directory_path).join(Path::new(output_filename));

    let input_contents =
        fs::read_to_string(input_full_path).expect("Failed to open the test file.");
    let output_contents =
        fs::read_to_string(output_full_path).expect("Failed to open the test file.");

    let input_lines: Vec<&str> = input_contents.lines().collect();
    let output_array: Vec<i32> = output_contents
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("{:?}", output_array);

    for line in input_lines.iter().skip(2) {
        let input_values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        println!("{:?}", input_values);
    }
}

fn main() {
    let tree = SegmentTree::new(5);
    tree.print_tree();
    run_tests()
}
