// use std::slice::Iter;
// use std::ops::Range;
// use std::io;
use rand::random;
use spmc::{channel, Sender, Receiver};
use core::iter::{ Iterator
                , Map
                // , Filter
                };


pub struct Branch<P>
where P       : Pipe,
      P::Item : Send,
{
    feed     : P,
    tx       : Sender<P::Item>,
    main     : PhantomSource<P::Item>,
    sideways : PhantomSource<P::Item>
}


impl<P> Branch<P>
where P       : Pipe,
      P::Item : Send,
{
    pub fn new<F>(feed : P, sideways : F) -> Branch<P>
    where F : Fn(P::Item) -> ()
    {
        let (tx, rx) = channel::<P::Item>();
        let main     = PhantomSource::new(rx.clone());
        let sideways = PhantomSource::new(rx.clone());

        Branch{feed, tx, main, sideways}
    }
}


pub struct PhantomSource<T : Send> {
    rx : Receiver<T>,
}

impl<T : Send> PhantomSource<T> {
    pub fn new(rx : Receiver<T>) -> Self {
        Self{rx}
    }
}

impl<T : Send> Iterator for PhantomSource<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rx.recv() {
            Ok (item) => Some(item),
            Err(_)    => None,
        }
    }

}

pub trait Pipe: Iterator {
    fn sink<C>(&mut self, fun : C) -> ()
        where Self : Sized,
              C    : Fn(Self::Item) -> (),
    {
        self.for_each(|x| fun(x));
    }

    fn branch<F>(self, pipe : F) -> Branch<Self>
    where Self       : Sized,
          Self::Item : Send,
          F          : Fn(Self::Item) -> ();
}

impl<O, I, F> Pipe for Map<I, F>
    where
        I : Iterator,
        F : FnMut(I::Item) -> O,
        O : Send,
{
    fn branch<B>(self, pipe : B) -> Branch<Self>
    where Self : Sized,
          B    : Fn(Self::Item) -> ()
    {
        Branch::new(self, pipe)
    }
}


impl<P> Iterator for Branch<P>
where P       : Pipe,
      P::Item : Send,
{
    type Item = P::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.feed.next();
        if let Some(item) = item {
            self.tx.send(item).unwrap();
        }
        None
    }
}



pub struct RandomNumbers {
    n : usize,
}


impl Iterator for RandomNumbers {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            self.n -= 1;
            Some(random())
        }
        else {
            None
        }
    }
}




///////////////////////////////////////////////////
/// Example
///////////////////////////////////////////////////

fn less_than_one_half(x : &f64) -> bool {
    *x < 0.5
}

fn square_the_number(x : f64) -> f64 {
    x.powf(2.0)
}

fn add_3(x : f64) -> f64 {
    x + 3.
}

fn print_to_std<T>(name : &'static str) -> impl Fn(T) -> ()
where T : std::fmt::Debug
{
    move |x : T| {
        println!("this is {:?}. received {:?}", name, x)
    }
}


fn main() -> () {
    let source = RandomNumbers{n : 10};
    let result = source.filter(less_than_one_half)
                       .map   (square_the_number)
                       .map   (add_3)
                       .sink  (print_to_std("sink"));

    println!("{:?}", result);
    ()
}
