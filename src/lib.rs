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

        if idx == QUESTIONS.len() - 1 && ans_dir.file_name() == Some(std::ffi::OsStr::new(QUESTIONS[idx])){
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


