// use std::slice::Iter;
// use std::ops::Range;
// use std::io;
use rand::random;
use core::iter::{ Iterator
                , Map
                // , Filter
                };


pub trait Source<T> : Iterator<Item = T> {
}

pub trait Pipe<T, C> : Iterator<Item = T> {
    fn sink(&mut self, fun : C) -> ()
    where Self : Sized,
          C    : Fn(T) -> (),
    {
        self.for_each(|x| fun(x));
    }

//    fn branch(&self, pipe : impl Pipe<T>) -> Branch<T>;

}

pub trait Drain<T> {
    fn drain(&mut self) -> ();
}

pub struct Sink<'a, C, T>
    where C : FnMut(T) -> ()
{
    pub fun  :  C,
    pub feed : Box<&'a mut dyn Pipe<T, C>>,

}

impl<'a, C, T> Drain<T> for Sink<'a, C, T>
    where C : FnMut(T) -> (),
{
    fn drain(&mut self) -> () {
        ()
    }
}

// struct Branch<T> {
//     sideways : Box<dyn Pipe<T>>,
// }


impl<T, I, F, C> Pipe<T, C> for Map<I, F>
    where
        I : Iterator,
        F : FnMut(I::Item) -> T,
        C : FnMut(I::Item) -> ()
{
    // fn branch(&self, pipe : impl Pipe<T>) -> Branch<T> {
    //
    // }
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
