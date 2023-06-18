use spmc::Sender;

use crate::liquid::Pipe;

pub struct Branched<'a, I>
    where I : Send
{
    pub(crate) upstream : Option<&'a mut dyn Pipe<Item = I>>,
    pub(crate) sender   : Sender<Option<I>>
}

impl<'a, I : Send + Copy> Iterator for Branched<'a, I>
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.upstream
                       .as_mut()
                       .unwrap()
                       .next  ();

        let sent = self.sender.send(item);
        assert!(sent.is_ok());

        item
    }
}

impl<'a, I : Send + Copy> Pipe for Branched<'a, I>
{}
