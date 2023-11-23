use segment_tree::SegmentTree;
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
            let mut output_index = 0;
            tree = SegmentTree::new(&input_values);
            tree.reset_lazy();
            println!("working on input: {}", input_filename);

            for line in all_input_values.iter().skip(2) {
                if line[0] == 0 {
                    println!("update {:?}", line);
                    tree.update(line[1] as usize, line[2] as usize, line[3]);
                } else if line[0] == 1 {
                    println!(
                        "{:?} {}",
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
    let mut tree = SegmentTree::new(&[18, 17, 13, 19, 15, 11, 20]);
    tree.update(1, 4, 5);
    println!("{} ", tree.max_query(1, 2));
}
