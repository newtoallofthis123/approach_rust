use crate::stream::Stream;

pub struct Node {
    pub content: Option<String>,
    pub render_id: usize,
    pub prerender: bool,
    pub nodes: Vec<Node>,
}

impl Stream for Node {
    fn RenderHead(&self) -> Vec<String> {
        vec!["".to_string()]
    }
    fn RenderTail(&self) -> Vec<String> {
        vec!["".to_string()]
    }
    fn RenderCorpus(&self) -> Vec<String> {
        let mut output = Vec::new();
        if let Some(content) = &self.content {
            output.push(content.clone());
        }
        if !self.prerender {
            for n in &self.nodes {
                output.extend_from_slice(&n.RenderHead());
                output.extend_from_slice(&n.RenderCorpus());
                output.extend_from_slice(&n.RenderTail());
            }
        }
        output
    }

    fn Render(&self) -> String {
        let mut output = match &self.content {
            Some(content) => content.clone(),
            None => "".to_string(),
        };
        for node in &self.nodes {
            output.push_str(&node.RenderHead().join(""));
            output.push_str(&node.RenderCorpus().join(""));
            output.push_str(&node.RenderTail().join(""));
        }
        output
    }

    fn toArray(&self) -> Vec<String> {
        let mut output = Vec::new();
        for node in &self.nodes {
            output.push(node.Render());
        }
        output
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.Render())
    }
}
