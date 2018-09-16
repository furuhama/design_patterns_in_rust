// Mediator pattern

use std::collections::HashMap;

pub fn mediator() {
    let mut mediator = Mediator::new();

    let label1 = String::from("Nyan");
    let label2 = String::from("Piyo");
    let node1 = Node::new(&label1);
    let node2 = Node::new(&label2);

    mediator.add_node(node1);
    mediator.add_node(node2);

    let n1 = mediator.get(&label1);
    n1.send_msg(&mediator, &label1, "hi from Nyan");
    let n2 = mediator.get(&label2);
    n2.send_msg(&mediator, &label2, "hi from Piyo");
}

struct Mediator {
    nodes: HashMap<String, Node>,
}

impl Mediator {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.label.clone(), node);
    }

    fn consult_to(&self, label: &str, msg: &str) {
        self.nodes[label].receive_msg(msg);
    }

    fn get(&self, label: &str) -> &Node {
        &self.nodes[label]
    }
}

struct Node {
    label: String,
}

impl Node {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }

    fn send_msg(&self, mediator: &Mediator, target_label: &str, msg: &str) {
        mediator.consult_to(target_label, msg);
    }

    fn receive_msg(&self, msg: &str) {
        println!("{} received message: {}", self.label, msg);
    }
}
