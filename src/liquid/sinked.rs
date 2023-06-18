use crate::liquid::Pipe;

pub struct Sinked<'a, I, F> {
    pub(crate) upstream : Option<&'a mut dyn Pipe<Item = I>>,
    pub(crate) f        : F,
}

impl <'a, I, F> Sinked<'a, I, F>
    where F : Fn(I) -> ()
{
    pub fn consume(&mut self) -> () {
        self.count();
    }
}

impl<'a, I, F> Iterator for Sinked<'a, I, F>
    where F : Fn(I) -> ()
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.upstream
            .as_mut()
            .unwrap()
            .next  ()
            .map   (&mut self.f)
            .map(|value| assert_eq!((), value))
    }
}

impl<'a, I, F> Pipe for Sinked<'a, I, F>
    where F : Fn(I) -> ()
{}
