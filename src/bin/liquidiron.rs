// use std::slice::Iter;
// use std::ops::Range;
// use std::io;
use rand::random;
use core::iter::{ Iterator
                , Map
                // , Filter
                };


pub struct Branch<P1, P2>
where P1 : Pipe,
      P2 : Pipe
{
    feed     : P1,
    sideways : P2,
}



pub trait Source : Iterator {
}

pub trait Pipe: Iterator {
    fn sink<C>(&mut self, fun : C) -> ()
        where Self : Sized,
              C    : Fn(Self::Item) -> (),
    {
        self.for_each(|x| fun(x));
    }

    fn branch<U>(self, pipe : U) -> Branch<Self, U>
    where Self : Sized,
          U    : Pipe;

}

impl<O, I, F> Pipe for Map<I, F>
    where
        I : Iterator,
        F : FnMut(I::Item) -> O,
{
    fn branch<P>(self, pipe : P) -> Branch<Self, P>
    where P : Pipe {
        Branch{ feed     : self
              , sideways : pipe
              }
    }
}


impl<P1, P2> Iterator for Branch<P1, P2>
where P1 : Pipe,
      P2 : Pipe,
{
    type Item = P1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.feed.next()
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

fn print_to_std<T : std::fmt::Debug>(x : T) -> () {
    println!("{:?}", x);
}


fn main() -> () {
    let source = RandomNumbers{n : 10};
    let result = source.filter(less_than_one_half)
                       .map   (square_the_number)
                       .map   (add_3)
                       .sink  (print_to_std);

    println!("{:?}", result);
    ()
}
