use segment_2::sweep;
use segment_2::SegmentTree;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    //First function to test the first assignemnt.
    #[test]
    fn run_tests() {
        let directory_path = "src/Testset_handson2_2324_p1/";
        let mut tree: SegmentTree;

        //Hardcoded, should read the number of txt files in the folder instead.
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

    //Second function to test the second assignemnt.
    #[test]
    fn test2() {
        let directory_path = "src/Testset_handson2_2324_p2/";

        //Hardcoded, should read the number of txt files in the folder instead.
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
    let mut tree = SegmentTree::new(leaves.len());
    tree.build(&leaves);
}
