use spmc::channel;
use std::thread;

use crate::liquid::BranchSource;
use crate::liquid::Mapped;
use crate::liquid::Filtered;
use crate::liquid::Sinked;
use crate::liquid::Branched;

pub trait Pipe : Iterator {
    fn apply <'a, F, O>(&'a mut self, f : F) ->   Mapped<'a, Self::Item, F>
    where F    : Fn(Self::Item) -> O,
          Self : Sized
    {
        Mapped{upstream : Some(self), f}
    }

    fn select<'a, F>(&'a mut self, f : F) -> Filtered<'a, Self::Item, F>
    where F    : Fn(&Self::Item) -> bool,
          Self : Sized
    {
        Filtered{upstream : Some(self), f}
    }

    fn sink  <'a, F>(&'a mut self, f : F) ->   Sinked<'a, Self::Item, F>
    where F    : Fn(Self::Item) -> (),
          Self : Sized
    {
        Sinked{upstream : Some(self), f}
    }

    fn branch<'a, F>(&'a mut self, f : F) -> Branched<'a, Self::Item>
    where Self::Item : 'static + Send,
          F          : 'static + Fn(BranchSource<Self::Item>) -> () + Send,
          Self       : Sized
    {
        let (sender, receiver) = channel();
        let source = BranchSource{receiver};

        thread::spawn(move || {
            f(source)
        });
        Branched{upstream : Some(self), sender}
    }
}
