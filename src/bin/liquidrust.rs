use rand::random;
use rustic::liquid::SourceTrait;
use rustic::liquid::Pipe;

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

impl SourceTrait for RandomNumbers {}




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
    let mut source = RandomNumbers{n : 100};
    let result = source.stream()
                       .select(less_than_one_half)
                       .apply (square_the_number)
                       .branch(|mut source| {
                           let f = print_to_std("branch");
                           source.apply  (square_the_number)
                                 .sink   (f)
                                 .consume()
                       })
                       .apply (add_3)
                       .sink  (print_to_std("sink"))
                       .count ();

    println!("{:?}", result);
    ()
}
