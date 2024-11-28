use aoc_tools::input::*;
use input::read_lines;

#[derive(Debug)]
struct Node {
    children: Vec<Box<Node>>,
    data: Vec<i64>
}

// Create the tree recursively
fn create_node(data: &Vec<i64>, curr_ind: &mut usize) -> Node {
    let mut node = Node{data: Vec::new(), children: Vec::new()};
    let num_children = data[*curr_ind].clone();
    let num_data = data[*curr_ind + 1] as usize;
    *curr_ind += 2;
    for _ in 0..num_children {
        let child = Box::new(create_node(data, curr_ind));
        node.children.push(child);
    }
    for i in 0..num_data {
        node.data.push(data[*curr_ind + i]);
    }
    *curr_ind += num_data;
    node
}

// Sum the metadata of each node in tree recursively
fn sum_metadata(node: &Node) -> i64 {
    let mut tot_metadata = 0i64;
    for child in node.children.iter() {
        tot_metadata += sum_metadata(&child);
    }
    tot_metadata + node.data.iter().sum::<i64>()
}

// Calculate the value of the given node (recursively add children values)
fn value_of_node(node: &Node) -> i64 {
    if node.children.len() == 0 {
        return node.data.iter().sum::<i64>();
    }
    let mut node_count = 0i64;
    for ind in node.data.iter() {
        if ind - 1 < 0 || ind - 1 >= node.children.len() as i64 {
            continue
        }
        node_count += value_of_node(&node.children[*ind as usize - 1]);
    }
    node_count
}

fn main() {
    let nodes: Vec<i64> = read_lines::<String>("input.txt")
        .unwrap()[0]
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut ind = 0usize;
    let tree = create_node(&nodes, &mut ind);

    println!("part1: {}", sum_metadata(&tree));
    println!("part2: {}", value_of_node(&tree));
}
