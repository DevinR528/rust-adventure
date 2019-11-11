//! serialize questions in struct iterate through deserialized for each ?
//!
use std::any::Any;
use std::env;
use std::io;
use std::panic::{self, AssertUnwindSafe};
use std::path::PathBuf;
use std::string::ToString;

use console::{style, Term};
use image::{GenericImageView, ImageFormat};
use termimage::{Error, ops};
use trybuild;

const QUESTIONS: &[&str] = &["first.rs", "second.rs", "third.rs", "fourth.rs"];

const MSGS: &[&str] = &[
    "Great job, on to invariance, covariance and contravariance!! just kidding",
    "Wow you're doing great!!",
    "Now you're ready for a pull request to the borrow checker!!",
    "Great keep up the good work."
];

fn celebrate(mut term: &mut Term, mut dir: PathBuf) -> Result<(), Error> {
    dir.push("rust-logo.png");

    let img = ops::load_image(&(dir.as_os_str().to_str().unwrap().to_string(), dir), ImageFormat::PNG)?;
    let (x, y) = term.size();
    let mut img_s = ops::image_resized_size(img.dimensions(), (x as u32, y as u32), true);
    img_s.0 *= 3;
    img_s.1 *= 3;
    let resized = ops::resize_image(&img, img_s);

    match term.features().colors_supported() {
        true => ops::write_ansi_truecolor(&mut term, &resized),
        false => ops::write_ansi(&mut term, &resized),
        // None => ops::write_no_ansi(&resized),
    };
    Ok(())
}

pub fn start_adventure() -> io::Result<()> {
    let mut ans_dir = env::current_dir()?;
    ans_dir.push("questions");

    let mut term = Term::stdout();

    for (idx, _) in QUESTIONS.iter().enumerate() {
        let mut test_pass: Result<(), Box<(dyn Any + Send)>> = Err(Box::new(0));

        term.clear_screen()?;
        while let Err(_) = test_pass {
            let msg = format!(
                "{}\n{}",
                style("Look in the `questions` folder to start then").cyan(),
                style("enter the name of the file to test from `questions` folder.").cyan(),
            );
            term.write_line(&msg)?;
            let file = term.read_line()?;
            ans_dir.push(file);

            test_pass = panic::catch_unwind(AssertUnwindSafe(|| {
                let try_test = trybuild::TestCases::new();
                try_test.pass(&ans_dir);
            }));
        }
        term.write_line(MSGS[idx])?;

        if idx == QUESTIONS.len() - 1 && ans_dir.file_name() == Some(std::ffi::OsStr::new(QUESTIONS[idx])) {
            let mut path_clone = ans_dir.clone();
            path_clone.pop();
            path_clone.pop();
            match celebrate(&mut term, path_clone) {
                Ok(_) => {},
                Err(e) => return Err(io::Error::from_raw_os_error(e.exit_value())),
            };
            term.read_line()?;
        }
        // remove user supplied file name
        ans_dir.pop();
    }
    Ok(())
}


mod test {
    use std::marker::PhantomData;

    // the all important invariant lifetime.
    #[derive(Copy, Clone, PartialEq, PartialOrd, Eq)]
    // 
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

            if false {
                struct Guard<'g>(&'g Lifetime<'g>);
                impl<'g> Drop for Guard<'g> {
                    fn drop(&mut self) {}
                }
                _guard = Guard(&id);
            }
        };
    }

    #[test]
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
    }
}
