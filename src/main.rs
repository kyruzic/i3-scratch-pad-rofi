extern crate i3ipc;
use i3ipc::I3Connection;
use i3ipc::reply::Node;
use std::fmt::Error;

fn main() {
    let mut connection = I3Connection::connect().unwrap();
    let layout_tree = connection.get_tree().unwrap();
    let nodes = get_scratch_pad_nodes(&layout_tree).unwrap();

    for n in nodes.iter() {
        println!("{:?}", n)
    }


}

fn is_scratch_node(node: &Node) -> bool {
    return match &node.name {
        Some(name) => {
            if name == "__i3_scratch" {
                true
            } else {
                false
            }
        },
        None => false
    }
}

fn get_scratch_pad_nodes(node: &Node) -> Result<&Vec<Node>, &str> {
    return match node.find_child(is_scratch_node) {
        Some(n) => Ok(&n.floating_nodes),
        None => Err("Failed to find scratch node, i3 bug")
    }
}
