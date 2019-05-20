use static_list::*;
use std::marker::PhantomData;
use std::fmt::Debug;


#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}

struct Demo<'a> {
    pub t: static_list_type![&'a Debug; u8, &'static str, [u32; 4], Person],
    _m: PhantomData<&'a u32>
}

impl <'a> Demo<'a> {
    pub fn new() -> Self {
        Self {
            t: static_list![1, "It is a string", [1,2,3,4], Person { name: "Andrey".into(), age: 30 }],
            _m: Default::default(),
        }
    }
}

fn main() {
    let demo = Demo::new();

    println!("{:?}", demo.t.iter().collect::<Vec<_>>());
}