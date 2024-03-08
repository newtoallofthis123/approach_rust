pub trait StreamTrait{
    type Item;

    fn new() -> Self;
    fn write(&mut self, item: Self::Item);
}
