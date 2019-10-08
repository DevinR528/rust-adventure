// Here we have two options, a hint: lifetime elision
// or fully explicit.

struct Stuff<'s> {
    thing_one: &str,
}

impl<'s> Stuff<'s> {
    
    fn new(thing: &str) -> Stuff<'s> {
    //        try removing this ^^^^
        Stuff { thing_one: thing, }
    }
}

fn main() { }
