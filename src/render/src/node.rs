pub trait Node{
    fn RenderHead(&mut self);
    fn RenderTail(&mut self);
    fn RenderCorpus(&mut self);
    fn render(&mut self) -> String;
    fn toArray(&self) -> Vec<String>;
}
