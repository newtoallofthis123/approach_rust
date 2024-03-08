pub trait Streamable {
    type Stream: Iterator;

    fn stream(&self, stream: Self::Stream);
    fn StreamHead(&self, stream: &mut Self::Stream);
    fn StreamTail(&self, stream: &mut Self::Stream);
    fn StreamCorpus(&self, stream: &mut Self::Stream);

    fn render(&self, stream: &Self::Stream) -> String;
    fn RenderHead(&self, stream: &mut Self::Stream)->Self::Stream;
    fn RenderTail(&self, stream: &mut Self::Stream)->Self::Stream;
    fn RenderCorpus(&self, stream: &mut Self::Stream)->Self::Stream;
}
