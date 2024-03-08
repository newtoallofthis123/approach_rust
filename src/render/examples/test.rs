#![allow(non_snake_case, non_camel_case_types)] // <3 :lol:

use render::stream::StreamTrait;

#[derive(Debug, Clone)]
struct Node {
    stream: nodes,
    content: Option<String>,
}

#[derive(Debug, Clone)]
struct nodes {
    nodes: Vec<Node>,
}

// We make nodes
impl Iterator for nodes {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.nodes.len(), Some(self.nodes.len()))
    }
}

impl StreamTrait for nodes {
    type Item = Node;

    fn new() -> Self {
        nodes { nodes: Vec::new() }
    }

    fn write(&mut self, item: Self::Item) {
        self.nodes.push(item);
    }
}

struct outputstream{
    stream: Vec<String>
}

impl Iterator for outputstream{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stream.len(), Some(self.stream.len()))
    }
}

impl StreamTrait for outputstream{
    type Item = String;

    fn new() -> Self {
        outputstream { stream: Vec::new() }
    }

    fn write(&mut self, item: Self::Item) {
        self.stream.push(item);
    }
}

impl Node {
    fn new(content: &str) -> Self {
        Node {
            stream: nodes::new(),
            content: Some(content.to_string()), 
        }        
    }

    fn StreamHead(&self, output: &mut outputstream){
        output.write("Head".to_string())
    }

    fn StreamTail(&self, output: &mut outputstream){
        output.write("Tail".to_string())
    }

    fn StreamCorpus(&self, output: &mut outputstream){
        if let Some(content) = &self.content {
            output.write(content.to_string())
        }

        for node in &self.stream.nodes{
            // this should be same as 
            // yield node.stream(self, outputstream); // you can't use 
            node.StreamHead(output);
            node.StreamCorpus(output);
            node.StreamTail(output);
        }
    }

    fn stream(&self, output: &mut outputstream){
        self.StreamHead(output);
        self.StreamCorpus(output);
        self.StreamTail(output);
    }

    fn RenderHead(&self, output: &mut outputstream) -> String{
        self.StreamHead(output);
        output.next().unwrap()
    }
    fn RenderTail(&self, output: &mut outputstream) -> String{
        self.StreamTail(output);
        output.next().unwrap()
    }
    fn RenderCorpus(&self, output: &mut outputstream) -> String{
        self.StreamCorpus(output);
        output.next().unwrap()
    }
    fn render(&self, output: &mut outputstream)-> String{
        self.stream(output);
        let head = self.RenderHead(output);
        let corpus = self.RenderCorpus(output);
        let tail = self.RenderTail(output);
        format!("{} {} {}", head, corpus, tail)
    }
}

fn main() {
    let mut node = Node::new("Hello");
    node.stream.write(Node::new("World"));
    let mut output = outputstream::new();
    let res = node.render(&mut output);
    println!("{:?}", res);
}
