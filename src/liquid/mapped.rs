use crate::liquid::pipe::Pipe;

pub struct Mapped<'a, I, F> {
    pub(crate) upstream : Option<&'a mut dyn Pipe<Item = I>>,
    pub(crate) f        : F,
}

impl<'a, I, F, O> Iterator for Mapped<'a, I, F>
    where F : Fn(I) -> O
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        self.upstream
            .as_mut()
            .unwrap()
            .next  ()
            .map   (&mut self.f)
    }
}

impl<'a, I, F, O> Pipe for Mapped<'a, I, F>
    where F : Fn(I) -> O
{}
