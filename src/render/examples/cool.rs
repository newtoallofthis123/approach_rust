//! This doesn't work

#![allow(non_snake_case, non_camel_case_types)] // <3 :lol:

use async_stream::stream;
use futures::{pin_mut, Stream, StreamExt};
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

impl Node {
    fn new(content: &str) -> Self {
        Node {
            stream: nodes::new(),
            content: Some(content.to_string()),
        }
    }

    fn _empty() -> Self {
        Node {
            stream: nodes::new(),
            content: None,
        }
    }

    fn StreamHead(&self) -> impl Stream<Item = Option<String>> {
        stream! {
            yield None
        }
    }

    fn StreamTail(&self) -> impl Stream<Item = Option<String>> {
        stream! {
            yield None
        }
    }

    fn StreamCorpus(self) -> impl Stream<Item = Option<String>> {
        stream! {
            for node in self.stream.nodes{
                for await item in node.stream(){
                    yield None;
                }
            }
        }
    }

    fn stream(&self) -> (impl Stream<Item = Option<String>>) {
        stream! {
            for await item in self.StreamHead(){
                yield item;
            }
            for await item in self.StreamCorpus(){
                yield item;
            }
            for await item in self.StreamTail(){
                yield item;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut node = Node::new("Hello");
    node.stream.write(Node::new("Child 1"));
    node.stream.write(Node::new("Child 2"));
    let stream = node.stream();
    pin_mut!(stream);
    while let Some(value) = stream.next().await {
        println!("got {:?}", value);
    }
}
