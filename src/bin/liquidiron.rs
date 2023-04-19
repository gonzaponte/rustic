use std::slice::Iter;
use std::ops::Range;
use std::io;
use rand::random;
use core::iter::{Iterator, Map, Filter};


pub trait Source<T> : Iterator<Item = T> {
}

pub trait Pipe<T> : Iterator<Item = T> {
    type Consumer;

    fn sink<'a>(&'a self, fun : Self::Consumer) -> Sink<'a, Self::Consumer, T>
    where Self           : Sized,
          Self::Consumer : Fn(T) -> (),
    {
        Sink{ fun, iter : Box::new(self) }
    }

//    fn branch(&self, pipe : impl Pipe<T>) -> Branch<T>;

}

pub trait Drain<T> {
    fn drain(&mut self) -> ();
}

pub struct Sink<'a, F, T>
    where F : Fn(T) -> ()
{
    fun  :  F,
    iter : Box<&'a dyn Pipe<T, Consumer = F>>,

}

impl<'a, F, T> Drain<T> for Sink<'a, F, T>
    where F : Fn(T) -> (),
{
    fn drain(&mut self) -> () {
        let x = *self.iter;
        for item in  x.into_iter(){
            (self.fun)(item);
        }
        ()
    }
}

// struct Branch<T> {
//     sideways : Box<dyn Pipe<T>>,
// }


impl<T, I, F> Pipe<T> for Map<I, F>
    where
        I : Iterator,
        F: FnMut(I::Item) -> T,
{
    type Consumer = F;

    // fn branch(&self, pipe : impl Pipe<T>) -> Branch<T> {
    //
    // }
}

pub struct RandomNumbers {
    n : usize,
}


// pub struct PrintToStd<T> {
//     f : Box<dyn Fn(T) -> io::Result<()>>
// }

// impl<T> Pipe<T> for PrintToStd<T> {
//     fn drain(&self) -> Option<T> {
//
//     }
// }


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

fn less_than_one_half(x : &f64) -> bool {
    *x < 0.5
}

fn square_the_number(x : f64) -> f64 {
    x.powf(2.0)
}

fn print_to_std<T : std::fmt::Debug>(x : T) -> () {
    println!("{:?}", x);
}

fn main() -> () {
    let source = RandomNumbers{n : 5};
    let result = source.filter(less_than_one_half)
                       .map   (square_the_number)
                       .sink  (print_to_std);

    // println!("{:?}", result);
    ()
}


//
//
// pub trait Source<'a, T> {
//     fn iter(&self) -> Iter<'a, T>;
// }
//
// impl<'a, T> Source<'a, T> for Range<T> {
//     fn iter(&self) -> Iter<'a, T> {
//         self.iter()
//     }
// }
//
// pub struct RandomNumbers {
//     number : Vec<String>,
// }
//
// impl<'a, T> Source<'a, T> for RandomNumbers {
//     fn iter(&self) -> Iter<'a, T> {
//         (0..10).iter()
//                .map(|_| random())
//     }
// }
