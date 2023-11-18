#[derive(Debug)]
struct SegmentTree {
    size: u32,
    tree: Vec<u32>,
}

impl SegmentTree {
    fn new(size: u32) -> Self {
        let tree = vec![0; 2 * size as usize];

        SegmentTree{size, tree}
    }

    fn init_tree(){

    }

    fn print_tree(&self) {
        println!("{:?}", self.tree);
    }
}

#[cfg(test)]
mod tests {
}

fn run_tests(){
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    let filename = "input0.txt";
    let directory_path = "src/Testset_handson2_2324_p1/";
    let full_path = PathBuf::from(directory_path).join(Path::new(filename));

    let contents = fs::read_to_string(full_path).expect("Failed to open the test file.");

    let lines: Vec<&str> = contents.lines().collect();

    for line in lines{
        let values: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        
        println!("{:?}", values);
        }
    }

fn main() {
    let tree = SegmentTree::new(5);
    tree.print_tree();
    run_tests()
}
