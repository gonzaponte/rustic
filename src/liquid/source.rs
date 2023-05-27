use spmc::Receiver;
use crate::liquid::pipe::Pipe;

pub trait SourceTrait : Iterator {
    fn stream<'a>(&'a mut self) -> Source<'a, Self::Item>
    where Self : Sized
    {
        Source{ iter : Some(self) }
    }
}

pub struct Source<'a, T> {
    iter : Option<&'a mut dyn Iterator<Item = T>>,
}

impl<'a, T> Iterator for Source<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .as_mut()
            .unwrap()
            .next()
    }

}

impl<'a, T> Pipe for Source<'a, T> {

}
