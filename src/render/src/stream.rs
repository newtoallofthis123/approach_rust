pub trait Stream {
    fn RenderHead(&self) -> Vec<String>;
    fn RenderCorpus(&self) -> Vec<String>;
    fn RenderTail(&self) -> Vec<String>;
    fn Render(&self) -> String;
    fn toArray(&self) -> Vec<String>;
}
