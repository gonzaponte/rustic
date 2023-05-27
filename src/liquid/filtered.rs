use crate::liquid::Pipe;

pub struct Filtered<'a, I, F> {
    pub(crate) upstream : Option<&'a mut dyn Pipe<Item = I>>,
    pub(crate) f        : F,
}

impl<'a, I, F> Iterator for Filtered<'a, I, F>
    where F : Fn(&I) -> bool
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.upstream
                           .as_mut()
                           .unwrap()
                           .next  ();
            match item {
                None => return None,
                Some(value) => if (self.f)(&value) {return Some(value)} else {continue},
            }
        }
    }
}

impl<'a, I, F> Pipe for Filtered<'a, I, F>
    where F : Fn(&I) -> bool
{}
