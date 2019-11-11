use std::marker::PhantomData;

// the all important invariant lifetime.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq)]
pub struct Lifetime<'id>(PhantomData<fn(&'id ()) -> &'id ()>);

#[allow(dead_code)]
pub fn make_invariant<'id>() -> Lifetime<'id> {
    Lifetime(PhantomData)
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq)]
pub struct IdxID<'id> {
    idx: usize,
    _id: Lifetime<'id>,
}

pub struct Container<'id, T> {
    _id: Lifetime<'id>,
    items: Vec<T>,
}
impl<'c, T> Container<'c, T> {
    #[allow(dead_code)]
    pub fn new(id: Lifetime<'c>) -> Container<'c, T> {
        
        Self {
            _id: id,
            items: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: T) -> IdxID<'c> {
        let idx = self.items.len();
        self.items.push(item);

        IdxID {
            idx,
            _id: self._id,
        }
    }
    #[allow(dead_code)]
    pub fn get(&self, i: IdxID<'c>) -> Option<&T> {
        self.items.get(i.idx)
    }
}

#[macro_export]
macro_rules! make_container {
    ($name:ident) => {
        let id = make_invariant();
        let _guard;

        let mut $name = Container::new(id);

        // this is compiled away but the borrow checker uses it to prevent
        // lifetimes created in the same scope from being unified 
        // (evaluated to the same lifetime).
        if false {
            struct Guard<'g>(&'g Lifetime<'g>);
            impl<'g> Drop for Guard<'g> {
                fn drop(&mut self) {}
            }
            _guard = Guard(&id);
        }
    };
}

fn main() {
    make_container!(arena_a);
    let i_1a = arena_a.push(0_usize);
    let i_2a = arena_a.push(1);

    make_container!(arena_b);
    let i_1b = arena_b.push(2_usize);
    let i_2b = arena_b.push(3);

    println!("{:?}", arena_a.get(i_1a));
    println!("{:?}", arena_b.get(i_2b));

    println!("{:?}", arena_b.get(i_1b));
    println!("{:?}", arena_a.get(i_2a));

    // println!("{:?}", arena_a.get(i_2b));
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // this fails because each `Container` has a unique lifetime attached to it
    // because of our `make_invariant` function which is enforced by the 
    // `Guard` `Drop` trait we implemented
}
