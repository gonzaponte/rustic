// use std::slice::Iter;
// use std::ops::Range;
// use std::io;
use rand::random;
use core::iter::{ Iterator
                , Map
                // , Filter
                };


pub struct Branch<P, F>
where P : Pipe,
      F : Fn(P::Item) -> (),
{
    feed     : P,
    sideways : F,
}


impl<P, F> Branch<P, F>
where P : Pipe,
      F : Fn(P::Item) -> (),
{
    pub fn new(feed : P, sideways : F) -> Branch<P, F>
    {
        Branch{feed, sideways}
    }
}

pub trait Pipe: Iterator {
    fn sink<C>(&mut self, fun : C) -> ()
        where Self : Sized,
              C    : Fn(Self::Item) -> (),
    {
        self.for_each(|x| fun(x));
    }

    fn branch<F>(self, pipe : F) -> Branch<Self, F>
    where Self       : Sized,
          Self::Item : Send,
          F          : Fn(Self::Item) -> (),;
}

impl<O, I, F> Pipe for Map<I, F>
    where
        I : Iterator,
        F : FnMut(I::Item) -> O,
        O : Send,
{
    fn branch<B>(self, pipe : B) -> Branch<Self, B>
    where Self : Sized,
          B    : Fn(Self::Item) -> (),
    {
        Branch::new(self, pipe)
    }
}


impl<P, F> Iterator for Branch<P, F>
where P       : Pipe,
      P::Item : Copy,
      F       : Fn(P::Item) -> (),
{
    type Item = P::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.feed.next();

        if let Some(item) = next_item {
            (self.sideways)(item);
        }
        next_item
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
    move |x : T| -> () {
        println!("this is {:?}. received {:?}", name, x)
    }
}


fn main() -> () {
    let source = RandomNumbers{n : 10};
    let result = source.filter(less_than_one_half)
                       .map   (square_the_number)
                       .branch(print_to_std("branch"))
                       .map   (add_3)
                       .sink  (print_to_std("sink"));

    println!("{:?}", result);
    ()
}
