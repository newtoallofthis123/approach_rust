use render::{node::Node, xml::Xml};

fn main(){
    let mut node = Xml::new("div");
    let another = Xml::new("p");

    node.setContent("Hello, World!");

    node.appendChild(another);
    println!("{}", node.render());
}
