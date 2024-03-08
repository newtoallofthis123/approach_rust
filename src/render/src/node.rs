pub trait NodeTrait{
    type Item;

    fn content(&self) -> Self::Item;
}
