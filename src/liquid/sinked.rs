use crate::liquid::pipe::Pipe;

pub struct Sinked<'a, I, F> {
    pub(crate) upstream : Option<&'a mut dyn Pipe<Item = I>>,
    pub(crate) f        : F,
}

impl<'a, I, F> Iterator for Sinked<'a, I, F>
    where F : Fn(I) -> ()
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let unit = self.upstream
                       .as_mut()
                       .unwrap()
                       .next  ()
                       .map   (&mut self.f)
                       .unwrap();
        assert_eq!((), unit);
        Some(())
    }
}

impl<'a, I, F> Pipe for Sinked<'a, I, F>
    where F : Fn(I) -> ()
{}
