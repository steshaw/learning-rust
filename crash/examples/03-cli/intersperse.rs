//
// FIXME: Fix the horrors here.
//
// TODO: Implement intersperse over iterators.
//
// ghci> intersperse 0 [1,2,3]
// [1,0,2,0,3]
//
// TODO: Is this `join` from the std? i.e. `xs.iter().join(0)`?
// TODO: Seems it might be available with +nightly.
//

use std::iter::Peekable;

// XXX: Is is better to use `T: Clone` or `T: Copy`
// XXX: Is is a way to avoid the additional constaint on T (using refs)?

#[derive(Debug)]
pub struct Intersperse<T, I>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    iter: Peekable<I>,
    t: T,
    inject: bool,
}

impl<T, I> Intersperse<T, I>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    fn new(iter: I, t: T) -> Intersperse<T, I> {
        Intersperse {
            iter: iter.peekable(),
            t,
            inject: false,
        }
    }
}

impl<T, I> Iterator for Intersperse<T, I>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.inject {
            let is_more = self.iter.peek().is_some();
            if is_more {
                self.inject = false;
                Some(self.t)
            } else {
                None
            }
        } else {
            self.inject = true;
            self.iter.next()
        }
    }
}

//
// Intersperse as an "extension method",
// with help from Jake Goulding (Shepmaster):
//
//   https://stackoverflow.com/a/30540952/482382
//
pub trait IntersperseExt {
    fn intersperse<T>(self, t: T) -> Intersperse<T, Self>
    where
        Self: Iterator<Item = T> + Sized,
        T: Copy,
    {
        Intersperse::new(self, t)
    }
}

impl<I: Iterator> IntersperseExt for I {}

//
// TODO: Generalise to non-Vec.
//
pub fn intersperse_vec<T: Copy>(a: T, xs: Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = vec![];
    let mut iter = xs.iter();
    let mut next = iter.next();
    while next.is_some() {
        result.push(*next.unwrap());
        next = iter.next();
        if next.is_some() {
            result.push(a)
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn some_tests() {
        println!("intersperse_vec:");
        let xs: Vec<u32> = vec![1, 2, 3];
        let ys = intersperse_vec(0, xs);
        assert_eq!(vec![1, 0, 2, 0, 3], ys);
        println!("ys = {:?}", ys);

        let xs = vec!["one", "two", "three"];
        let ys = intersperse_vec("boop", xs);
        assert_eq!(vec!["one", "boop", "two", "boop", "three"], ys);
        println!("ys = {:?}", ys);

        let one = "one".to_string();
        let two = "two".to_string();
        let three = "three".to_string();
        let xs = vec![&one, &two, &three];
        let boop = "boop".to_string();
        let ys: Vec<&String> = intersperse_vec(&boop, xs);
        let expected: Vec<&String> = vec![&one, &boop, &two, &boop, &three];
        assert_eq!(expected, ys);
        println!("ys = {:?}", ys);

        println!();
        println!("intersperse:");
        let xs = vec![1, 2, 3];
        let ys: Vec<i32> = xs.iter().intersperse(&0).copied().collect();
        assert_eq!(vec![1, 0, 2, 0, 3], ys);
        println!("ys = {:?}", ys);

        let xs = vec!["one", "two", "three"];
        let ys = xs.iter().intersperse(&"boop").copied().collect::<Vec<_>>();
        assert_eq!(vec!["one", "boop", "two", "boop", "three"], ys);
        println!("ys = {:?}", ys);

        let xs = vec!["one", "two", "three"];
        let xs: Vec<String> = xs.iter().map(|&s| s.to_string()).collect();
        let ys: Vec<String> = xs
            .iter()
            .intersperse(&"boop".to_string())
            .map(|s| s.to_string())
            .collect();
        let expected = vec!["one", "boop", "two", "boop", "three"];
        let expected = expected
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(expected, ys);
        println!("ys = {:?}", ys);
    }
}