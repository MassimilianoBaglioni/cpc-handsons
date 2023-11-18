#[derive(Debug)]
struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}
#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
}

/// This a representation of a tree.
/// Every node has an implicity id, which is its position on the vector `nodes`.
/// Every node has a key and at most two children. The ids of the children are
/// stored in `id_left` and `id_right`. These ids are `None` iff the child does not exit.
impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left child of the node `parent_id`
    /// iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    //First assignemnt solution//
    fn is_bst(&self, root: Option<usize>) -> bool {
        let mut prev: Option<u32> = None;
        self.is_bst_inorder(root, &mut prev)
    }

    // If the values are not returned in increasing order then it's not a BST.
    fn is_bst_inorder(&self, root_id: Option<usize>, prev: &mut Option<u32>) -> bool {
        if let Some(id) = root_id {
            //If root_id not None then:
            if !self.is_bst_inorder(self.nodes[id].id_left, prev) {
                return false;
            }

            if let Some(prev_key) = prev {
                if self.nodes[id].key <= *prev_key {
                    return false;
                }
            }

            *prev = Some(self.nodes[id].key);
            return self.is_bst_inorder(self.nodes[id].id_right, prev);
        }

        //If root_id is None then:
        true
    }
    //Notes: the above solution requires O(N) time complexity since uses inorder trasversal and O(1) extra space. There's another solution that takes O(N) extra space, which uses an extra array to store values received from the inorder trasversal. After storing values in the array the BST check can be done by checking if the array values are in ascending order since inorder trasversal returns values in ascending order in a BST. iF values are not ordered then the tree is not a BST.

    fn is_balanced(&self, root_id: Option<usize>) -> bool {
        self.is_balanced_rec(root_id) != 0
    }

    //A tree is balanced if for each of its nodes the difference between its left and right subtree is at maximum 1. We check this condition by doing an inorder trasversal, 1 is returned when we find the first null children. If we don't satisfy the basical balancing condition we just return 0. If we return a number (the height of the tree), then the tree is balanced otherwise we will return 0.
    fn is_balanced_rec(&self, root_id: Option<usize>) -> i32 {
        if let Some(id) = root_id {
            let left_height = self.is_balanced_rec(self.nodes[id].id_left);

            if left_height == 0 {
                return 0;
            }

            let right_height = self.is_balanced_rec(self.nodes[id].id_right);

            if right_height == 0 {
                return 0;
            }

            if (left_height - right_height).abs() > 1 {
                return 0;
            } else {
                return std::cmp::max(left_height, right_height) + 1;
            }
        }
        1
    }

    //Left nodes are equal to 2 * parent_node_index + 1, right nodes are 2 * parent_node_index + 2. If we are in a complete/semicomplete tree the number of nodes will be equal to the rightmost node's index.
    fn is_complete(&self, root: Option<usize>, index: usize) -> bool {
        if let Some(root_id) = root {
            if index >= self.nodes.len() {
                return false;
            } else {
                return self.is_complete(self.nodes[root_id].id_left, 2 * index + 1)
                    && self.is_complete(self.nodes[root_id].id_right, 2 * index + 2);
            }
        }
        true
    }

    //We assume the tree is complete since we do that check in another function that specifically designed for that.
    fn is_heap_util(&self, root: Option<usize>) -> bool {
        if let Some(root_id) = root {
            //No childred node -> return true
            if self.nodes[root_id].id_left.is_none() && self.nodes[root_id].id_right.is_none() {
                return true;
            }

            if let Some(id_left) = self.nodes[root_id].id_left {
                if self.nodes[root_id].id_right.is_none() {
                    return self.nodes[root_id].key >= self.nodes[id_left].key;
                } else {
                    if let Some(id_right) = self.nodes[root_id].id_right {
                        if self.nodes[root_id].key >= self.nodes[id_left].key
                            && self.nodes[root_id].key >= self.nodes[id_right].key
                        {
                            //Max heap property satisfied, check next nodes.
                            return self.is_heap_util(self.nodes[root_id].id_left)
                                && self.is_heap_util(self.nodes[root_id].id_right);
                        } else {
                            //Current node is less or equal than its childredn nodes.
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    //Function that simply calls for the two other functions that check the heap property.
    fn is_heap(&self, root: Option<usize>) -> bool {
        self.is_complete(root, 0) && self.is_heap_util(root)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }
    #[test]
    fn test_is_bst() {
        //Root only tree
        let mut tree = Tree::with_root(1);
        assert!(tree.is_bst(Some(0)));

        //One left child only tree, but NOT BST.
        tree.add_node(0, 5, true);
        assert!(!tree.is_bst(Some(0)));

        //One right child only tree, but NOT BST.
        let mut tree = Tree::with_root(5);
        tree.add_node(0, 1, false);
        assert!(!tree.is_bst(Some(0)));

        //One right child only tree is BST.
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 5, false);
        assert!(tree.is_bst(Some(0)));

        //One left child only tree is BST.
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 0, true);
        assert!(tree.is_bst(Some(0)));

        //Valid BST.
        tree = Tree::with_root(40);
        tree.add_node(0, 30, true); //id 1
        tree.add_node(0, 50, false); //id 2
        tree.add_node(1, 25, true); //id 4
        tree.add_node(1, 35, false); //id 5
        tree.add_node(2, 45, true); //id 6
        tree.add_node(2, 60, false); //id 7
        assert!(tree.is_bst(Some(0)));

        //Left only branches is a BST.
        tree = Tree::with_root(45);
        tree.add_node(0, 15, true);
        tree.add_node(1, 10, true);
        tree.add_node(2, 2, true);
        tree.add_node(3, 1, true);
        assert!(tree.is_bst(Some(0)));

        //Right only branches is a BST.
        tree = Tree::with_root(45);
        tree.add_node(0, 60, false);
        tree.add_node(1, 70, false);
        tree.add_node(2, 80, false);
        tree.add_node(3, 90, false);
        assert!(tree.is_bst(Some(0)));

        //Not a valid BST. Inner nodes fault.
        tree = Tree::with_root(40);
        tree.add_node(0, 30, false); //id 1
        tree.add_node(0, 50, true); //id 2
        tree.add_node(1, 25, true); //id 4
        tree.add_node(1, 35, false); //id 5
        tree.add_node(2, 45, true); //id 6
        tree.add_node(2, 60, false); //id 7
        assert!(!tree.is_bst(Some(0)));

        //Not a valid BST. Leaves nodes fault.
        tree = Tree::with_root(40);
        tree.add_node(0, 30, false); //id 1
        tree.add_node(0, 50, true); //id 2
        tree.add_node(1, 25, true); //id 4
        tree.add_node(1, 35, false); //id 5
        tree.add_node(2, 45, false); //id 6
        tree.add_node(2, 60, true); //id 7
        assert!(!tree.is_bst(Some(0)));

        //Not a valid BST. Same values nodes.
        tree = Tree::with_root(40);
        tree.add_node(0, 30, false); //id 1
        tree.add_node(0, 50, true); //id 2
        tree.add_node(1, 25, true); //id 4
        tree.add_node(1, 35, false); //id 5
        tree.add_node(2, 50, false); //id 6
        tree.add_node(2, 60, true); //id 7
        assert!(!tree.is_bst(Some(0)));

        //Valid BST. Example from wikipedia
        tree = Tree::with_root(8);
        tree.add_node(0, 3, true);
        tree.add_node(0, 10, false);
        tree.add_node(1, 1, true);
        tree.add_node(1, 6, false);
        tree.add_node(2, 14, false);
        tree.add_node(5, 13, true);
        tree.add_node(4, 4, true);
        tree.add_node(4, 7, false);

        assert!(tree.is_bst(Some(0)));
    }

    #[test]
    fn test_is_balanced() {
        let mut tree = Tree::with_root(20);
        //Not balanced.
        tree.add_node(0, 10, true);
        tree.add_node(0, 70, false);
        tree.add_node(2, 80, false);
        tree.add_node(3, 90, false);
        assert!(!tree.is_balanced(Some(0)));

        //Balanced.
        tree = Tree::with_root(20);
        tree.add_node(0, 10, true);
        tree.add_node(0, 70, false);
        tree.add_node(1, 12, true);
        assert!(tree.is_balanced(Some(0)));

        //Root only tree.
        tree = Tree::with_root(20);
        assert!(tree.is_balanced(Some(0)));

        //None root is considered balanced.
        assert!(tree.is_balanced(None));

        //Right branches only tree unbalanced.
        tree = Tree::with_root(10);
        tree.add_node(0, 20, false);
        tree.add_node(1, 30, false);
        tree.add_node(2, 60, false);
        assert!(!tree.is_balanced(Some(0)));

        //Left braches only tree unbalanced.
        tree = Tree::with_root(60);
        tree.add_node(0, 30, true);
        tree.add_node(1, 20, true);
        tree.add_node(2, 10, true);
        assert!(!tree.is_balanced(Some(0)));

        //One left child only tree.
        tree = Tree::with_root(20);
        tree.add_node(0, 10, true);
        assert!(tree.is_balanced(Some(0)));

        //One right child only tree.
        tree = Tree::with_root(20);
        tree.add_node(0, 30, false);
        assert!(tree.is_balanced(Some(0)));

        //Unbalanced tree with three nodes.
        tree = Tree::with_root(20);
        tree.add_node(0, 10, true);
        tree.add_node(1, 15, false);
        assert!(!tree.is_balanced(Some(0)));

        tree = Tree::with_root(50);
        tree.add_node(0, 17, true);
        tree.add_node(0, 76, false);
        tree.add_node(1, 9, true);
        tree.add_node(1, 23, false);
        tree.add_node(2, 54, true);
        tree.add_node(5, 72, false);
        tree.add_node(6, 67, true);
        tree.add_node(3, 14, false);
        tree.add_node(8, 12, false);
        tree.add_node(4, 19, true);
        assert!(!tree.is_balanced(Some(0)));
    }

    #[test]
    fn test_is_complete() {
        //Not complete tree.
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 8, true);
        tree.add_node(0, 6, false);
        tree.add_node(1, 7, false);

        assert!(!tree.is_complete(Some(0), 0));

        //Complete tree.
        tree = Tree::with_root(10);
        tree.add_node(0, 8, true);
        tree.add_node(0, 6, false);
        tree.add_node(1, 5, true);
        tree.add_node(1, 7, false);

        assert!(tree.is_complete(Some(0), 0));

        //Null passed considered complete.
        assert!(tree.is_complete(None, 0));

        //One left child only.
        tree = Tree::with_root(10);
        tree.add_node(0, 20, true);
        assert!(tree.is_complete(Some(0), 0));

        //One right child only.
        tree = Tree::with_root(10);
        tree.add_node(0, 20, false);
        assert!(!tree.is_complete(Some(0), 0));

        //Two levels unbalanced to the left.
        tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(1, 1, true);
        assert!(!tree.is_complete(Some(0), 0));

        //Two levels unbalanced to the right.
        tree = Tree::with_root(10);
        tree.add_node(0, 15, false);
        tree.add_node(1, 20, false);
        assert!(!tree.is_complete(Some(0), 0));
    }

    #[test]
    fn test_is_heap() {
        //Simple three nodes max heap.
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 7, false);
        assert!(tree.is_heap(Some(0)));

        //Null root. Considered valid.
        assert!(tree.is_heap(None));

        //Root only tree.
        tree = Tree::with_root(10);
        assert!(tree.is_heap(Some(0)));

        //Violating the max property of the heap.
        tree = Tree::with_root(20);
        tree.add_node(0, 50, true);
        tree.add_node(0, 10, false);
        assert!(!tree.is_heap(Some(0)));

        //Not complete tree.
        tree = Tree::with_root(20);
        tree.add_node(0, 10, true);
        tree.add_node(1, 8, true);
        tree.add_node(2, 5, true);
        assert!(!tree.is_heap(Some(0)));

        //Valid heap.
        tree = Tree::with_root(30);
        tree.add_node(0, 25, true);
        tree.add_node(0, 26, false);
        tree.add_node(1, 12, true);
        tree.add_node(1, 11, false);
        tree.add_node(2, 15, false);
        tree.add_node(2, 13, true);
        assert!(tree.is_heap(Some(0)));

        //Not valid heap.
        tree = Tree::with_root(50);
        tree.add_node(0, 10, true);
        tree.add_node(0, 20, false);
        tree.add_node(1, 60, false);
        assert!(!tree.is_heap(Some(0)));

        //Valid heap, exmaple from wikipedia.
        tree = Tree::with_root(100);
        tree.add_node(0, 19, true);
        tree.add_node(0, 36, false);
        tree.add_node(1, 17, true);
        tree.add_node(1, 3, false);
        tree.add_node(2, 25, true);
        tree.add_node(2, 1, false);
        tree.add_node(3, 2, true);
        tree.add_node(3, 7, false);
        assert!(tree.is_heap(Some(0)));
    }
}

fn main() {
    let mut tree = Tree::with_root(10);
    tree.add_node(0, 5, true);
    tree.is_heap(Some(0));
    tree.is_balanced(Some(0));
    tree.is_bst(Some(0));
    tree.sum();
}

//Author Massimiliano Baglioni
