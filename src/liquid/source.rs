use spmc::Receiver;
use crate::liquid::Pipe;

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

pub struct BranchSource<T : Send> {
    pub(crate) receiver : Receiver<Option<T>>,
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

impl<T : Send> Iterator for BranchSource<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().unwrap()
    }

}

impl<'a, T> Pipe for Source<'a, T> {

}

impl<T : Send> Pipe for BranchSource<T> {

}
