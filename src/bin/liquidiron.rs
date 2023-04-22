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
    fn sink<'a>(&'a mut self, fun : C) -> Sink<'a, C, T>
    where Self : Sized,
          C    : FnMut(T) -> (),
    {
        Sink{ fun, feed : Box::new(self) }
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
        // self.feed.inspect(self.fun);
        // Iterator::for_each(self.feed, self.fun);
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
