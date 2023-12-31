#[cfg(test)]
mod tests {
    #[test]
    fn readfile() {
        use super::max_vacation;
        use std::fs;
        use std::path::Path;
        use std::path::PathBuf;

        let directory_path = "src/TestSet/";

        for i in 0..=4 {
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

            let mut input_values: Vec<Vec<i32>> = Vec::new();

            for line in input_lines.iter() {
                let tmp_list: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                input_values.push(tmp_list);
            }
            let days = input_values[0][1];
            input_values.remove(0);
            assert!(max_vacation(input_values, days as usize) == output_array[0]);
        }
    }
    #[test]

    fn readfile2() {
        use super::dp_lsi;
        use std::fs;
        use std::path::Path;
        use std::path::PathBuf;

        let directory_path = "src/TestSet2/";

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

            let mut input_values: Vec<Vec<i32>> = Vec::new();

            for line in input_lines.iter() {
                let tmp_list: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                input_values.push(tmp_list);
            }
            input_values.remove(0);
            input_values.retain(|inner_list| !inner_list.is_empty()); // Tests work even with empty lines between rows with this.
            assert!(dp_lsi(input_values) == output_array[0]);
        }
    }
}

fn max_vacation(matrix: Vec<Vec<i32>>, days: usize) -> i32 {
    let row_size = matrix.len();
    let col_size = matrix[0].len();
    let mut dp_matrix = vec![vec![0; col_size + 1]; row_size + 1];

    for i in 1..row_size + 1 {
        for j in 1..col_size + 1 {
            let mut max = -1;
            let mut row = 0;
            for k in 0..j {
                row += matrix[i - 1][k];
            }
            dp_matrix[i][j] = row;
            let mut crt = row;
            for k in (0..j).rev() {
                crt -= matrix[i - 1][k];
                max = crt
                    .max(max)
                    .max(dp_matrix[i][j])
                    .max(crt + dp_matrix[i - 1][j - k]);
            }
            dp_matrix[i][j] = max;
        }
    }
    dp_matrix[row_size][days]
}

fn dp_lsi(mut array: Vec<Vec<i32>>) -> i32 {
    array.sort_by(|a, b| {
        let cmp = a[0].cmp(&b[0]);
        if cmp != std::cmp::Ordering::Equal {
            return cmp;
        }

        a[1].cmp(&b[1])
    });

    let mut lis = vec![1; array.len()];

    for i in 1..=array.len() - 1 {
        for j in 0..i {
            if array[i][1] > array[j][1] && lis[i] <= lis[j] && array[i][0] > array[j][0] {
                lis[i] = lis[j] + 1;
            }
        }
    }

    if let Some(max_value) = lis.iter().max() {
        *max_value
    } else {
        -1
    }
}

fn main() {}

//Modified version of binary search to insert items in log n instead of n.
// fn binary_search(arr: &mut Vec<i32>, target: i32) -> Option<usize> {
//     let mut low = 0;
//     let mut high = arr.len() - 1;
//     let mut mid = 0;

//     while low <= high {
//         mid = low + (high - low) / 2;
//         if arr[mid] == target {
//             return Some(mid);
//         } else if arr[mid] < target {
//             low = mid + 1;
//         } else {
//             if mid == 0 {
//                 break;
//             }
//             high = mid - 1;
//         }
//     }

//     // Use the binary search result to find the position to replace the next greater item.
//     let insert_index = if arr[mid] < target { mid + 1 } else { mid };

//     if insert_index < arr.len() && arr[insert_index] == target {
//         return Some(insert_index);
//     } else if insert_index >= arr.len() {
//         arr.push(target);
//     } else {
//         arr[insert_index] = target;
//     }
//     None
// }

// //Sort by the first index, if two values are equal sort by the second key.
// fn get_diff_sorted(mut list_of_lists: Vec<Vec<i32>>) -> Vec<i32> {
//     list_of_lists.sort_by(|a, b| {
//         let cmp = a[0].cmp(&b[0]);
//         if cmp != std::cmp::Ordering::Equal {
//             return cmp;
//         }

//         a[1].cmp(&b[1])
//     });

//     list_of_lists.into_iter().map(|pair| pair[1]).collect()
// }

// //This fucntion sorts the given list, then finds the longest increasing subsequence using the binary search version.
// fn lsi(mut list_of_lists: Vec<Vec<i32>>) -> usize {
//     let mut diff_vec = get_diff_sorted(list_of_lists); //N log n time coplexity for sorting, actually 2 sorts.
//     let mut lsi_vec = vec![i32::MAX]; //N space complexity.

//     for i in diff_vec {
//         //N time complexity
//         binary_search(&mut lsi_vec, i); //log n time complexity.
//     }
//     lsi_vec.len()
// }
