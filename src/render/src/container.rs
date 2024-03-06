use crate::node::Node;

enum ContainerReturn{
    String(String),
    Array(Vec<ContainerReturn>),
}

struct Container{
    prerender: bool,
    nodes: Vec<Node>,
}
