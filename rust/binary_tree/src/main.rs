struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn make(value: u32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
        Some(Box::new(Node { value, left, right }))
    }
}

// fn push_node(node: &Node, mut res: &Vec<u32>) -> Vec<u32> {
//     let clone = res.clone();
//     let copy = clone.push(node.value);
//     clone
// }

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    result.push(root.value); // [2];

    match (&root.left, &root.right) {
        (Some(l), Some(r)) => {
            let l2 = tree_by_levels(l);
            let r2 = tree_by_levels(r);
            let l2_first = l2.first().unwrap();
            let l2_rest = l2[1..].to_vec();

            let r2_first = r2.first().unwrap();
            let r2_rest = r2[1..].to_vec();

            result.push(*l2_first);
            result.push(*r2_first);

            result = [result, l2_rest, r2_rest].concat();
        }
        (Some(l), None) => {
            let l2 = tree_by_levels(l);
            result = [result, l2].concat();
        }
        (None, Some(r)) => {
            let r2 = tree_by_levels(r);
            result = [result, r2].concat();
        }
        (None, None) => {}
    };

    result
}

fn main() {
    // // let left_one = Node::make(1, None, None);
    // // let right_three = Node::make(3, None, None);
    // // let left_eight = Node::make(8, left_one, right_three);

    // // let left_four = Node::make(4, None, None);
    // // let right_five = Node::make(5, None, None);
    // // let right_nine = Node::make(9, left_four, right_five);

    // // let two = Node::make(2, left_eight, right_nine);

    // let r = tree_by_levels(&two.unwrap());

    let right_41 = Node::make(41, None, None);
    let left_38 = Node::make(38, None, None);
    let right_44 = Node::make(44, left_38, right_41);

    let left_20 = Node::make(20, None, None);
    let left_1 = Node::make(1, left_20, None);

    let right_40 = Node::make(40, left_1, right_44);
    let left_38 = Node::make(38, None, right_40);

    let right_36 = Node::make(36, None, None);

    let left_46 = Node::make(46, None, None);
    let right_2 = Node::make(2, None, None);
    let left_2 = Node::make(2, left_46, right_2);

    let right_23 = Node::make(23, left_2, right_36);
    let right_3 = Node::make(3, None, right_23);

    let right_35 = Node::make(35, left_38, right_3);

    let main_45 = Node::make(45, None, right_35);

    let result = tree_by_levels(&main_45.unwrap());
    println!(
        "expected : {:?}",
        [45, 35, 38, 3, 40, 23, 1, 44, 2, 36, 20, 38, 41, 46, 2]
    );
    println!("received: {:?}", result);
}
