use std::cell::{Ref,RefMut,  RefCell};
use std::fmt;
use std::fs;
use std::collections::HashMap;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("File access error");
    let tree = run_part1(contents);
    run_part2(tree)
}

fn run_part1(node_list:String) -> HashMap<String, RefCell<Node>> {
   let mut nodes:HashMap<String, RefCell<Node>> = HashMap::new();
   for node_info in node_list.lines() {
       let node = Node::new(node_info);
        nodes.insert(node.name.clone(), RefCell::new(node));
   }

   let mut count = 0;
   for (_, v) in &nodes {
        count += v.borrow_mut().get_depth(&nodes);
   }
   println!("{}", count);
   nodes
}

fn run_part2(tree:HashMap<String,RefCell<Node>>) {
    let mut my_node:RefMut<Node> = tree.get("YOU").unwrap().borrow_mut(); 
    let mut santas_node:RefMut<Node> = tree.get("SAN").unwrap().borrow_mut();

    let my_ancestors = my_node.get_ancestors(&tree);
    let santas_ancestors = santas_node.get_ancestors(&tree);

    let mut deepest_common_parent_name = String::new();
    for (my_par, san_par) in my_ancestors.iter().rev().zip( santas_ancestors.iter().rev()) {
        if my_par == san_par {
            deepest_common_parent_name = my_par.clone();
        } else {
            break;
        }
    } 

    let mut deepest_common_parent:RefMut<Node> = tree.get(&deepest_common_parent_name).unwrap().borrow_mut();

    let path_len = my_node.get_depth(&tree) + santas_node.get_depth(&tree) - 2*deepest_common_parent.get_depth(&tree) - 2;
    println!("{}", path_len);
}


struct Node {
    parent_name:String,
    name:String,
    depth:Option<i32>,
}

impl Node {
    fn new( string:&str ) -> Node {
        let parts:Vec<&str> = string.split(')').collect();
        
        let depth;
        if parts[0] == "COM" {
            depth = Option::Some(1)
        } else {
            depth = Option::None
        }

        Node {
            parent_name:String::from(parts[0]),
            name:String::from(parts[1]),
            depth:depth,
        }
    }

    fn get_depth(&mut self, parents_map:&HashMap<String, RefCell<Node>>) -> i32 {
        let result:i32;
        match self.depth {
            Option::Some(value) => {result = value},
            Option::None => {
                let parentcell = parents_map.get(&self.parent_name).unwrap();
                result = parentcell.borrow_mut().get_depth(parents_map) + 1;
                self.depth = Some(result);
            }
        }
        result
    }

    fn get_ancestors(&self, parents_map:&HashMap<String, RefCell<Node>>) -> Vec<String> {
        let mut ancestors:Vec<String> = Vec::new();
        let mut parent_node = parents_map.get(&self.parent_name).unwrap();
        let mut parent_data:Ref<Node> = parent_node.borrow();
        
        while parent_data.parent_name != "COM" {
            ancestors.push(parent_data.parent_name.clone()); // add self
            parent_node = parents_map.get(&parent_data.parent_name).unwrap();
            parent_data = parent_node.borrow();
        }
        ancestors
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}){} d:{}", self.parent_name, self.name, self.depth.unwrap())
    }
}