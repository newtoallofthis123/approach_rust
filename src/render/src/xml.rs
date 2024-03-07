use std::collections::{HashMap, HashSet};

use crate::node::Node;

#[derive(Debug, Clone)]
pub struct Xml {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub nodes: Vec<Xml>,
    pub content: String,
    pub stream: Vec<String>,
}

impl Node for Xml {
    fn RenderHead(&mut self) {
        self.stream.push(format!("<{}", self.tag));
        self.attributes.iter().for_each(|(k, v)| {
            self.stream.push(format!(" {}=\"{}\"", k, v));
        });
        self.stream.push(">".to_string());
    }

    fn RenderTail(&mut self) {
        self.stream.push(format!("</{}>", self.tag));
    }

    fn RenderCorpus(&mut self) {
        if !self.content.is_empty() {
            self.stream.push(self.content.clone());
        }
        println!("{:?}", self.nodes);
        self.nodes.iter_mut().for_each(|node| {
            node.RenderHead();
            node.RenderCorpus();
            node.RenderTail();
            self.stream.append(&mut node.stream);
        });
    }

    fn render(&mut self) -> String {
        self.RenderHead();
        self.RenderCorpus();
        self.RenderTail();

        self.stream.join("")
    }

    fn toArray(&self) -> Vec<String> {
        todo!()
    }
}

impl Xml {
    pub fn new(tag: &str) -> Self {
        Xml {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            nodes: Vec::new(),
            content: String::new(),
            stream: Vec::new(),
        }
    }

    pub fn setAttribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn setContent(&mut self, content: &str) {
        self.content = content.to_string();
    }

    pub fn appendChild(&mut self, node: Xml) {
        self.nodes.push(node);
    }
}
