// Here we have two options, a hint: lifetime elision
// or fully explicit. Rust's compiler is in general really smart
// just give it a chance.

struct Stuff<> {
    thing_one: &str,
}

impl<'a> Stuff<'a> {
    
    fn new(thing: &str) -> Stuff<'a> {
    //        try removing this ^^^^
        Stuff { thing_one: thing, }
    }
}

fn main() { }

