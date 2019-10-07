//! serialize questions in struct iterate through deserialized for each ?
//!
use std::any::Any;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use console::{Color, Term};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};

use trybuild;

mod ques;

const QUESTIONS: &[&str] = &["first.rs", "second.rs", "third.rs"];
const MSGS: &[&str] = &[
    "Great job, on to invariance, covariance and contravariance!! just kidding",
    "Wow you're doing great!!",
    "Now you're ready for a pull request to the borrow checker!!",
];

pub fn serialize_all_questions() -> io::Result<()> {
    let mut dir = env::current_dir()?;
    dir.push("questions");

    for file in QUESTIONS.iter() {
        dir.push(file);
        let code = fs::read_to_string(&dir)?;
        let question_obj = ques::QuestionHolder::build(code, &dir);
        write_new_question(&dir, question_obj)?;
        dir.pop();
    }
    Ok(())
}

fn write_new_question<T: serde::Serialize, P: AsRef<Path>>(file: P, item: T) -> io::Result<()> {
    let mut path = PathBuf::from(file.as_ref());

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    let question = serde_json::to_string(&item)?;
    encoder.write_all(question.as_ref())?;
    encoder.write_all(b"\n")?;
    let gz = encoder.finish()?;


    println!("{:?}", path);
    fs::write(file, gz)?;
    Ok(())
}

pub fn start_adventure() -> io::Result<()> {
    let try_test = trybuild::TestCases::new();

    let mut dir = env::current_dir()?;
    dir.push("questions");
    let term = Term::stdout();

    for (idx, q) in QUESTIONS.iter().enumerate() {
        let mut test_pass: Result<(), Box<(dyn Any + Send)>> = Err(Box::new(0));

        while let Err(_) = test_pass {
            // add then file name
            dir.push(q);
            let q_builder = ques::QuestionHolder::new(&dir)?;

            fs::DirBuilder::new()
                .recursive(true)
                .create(q_builder.folder().expect("DirBuilder failed, no parent in paht"))?;

            let mut start_file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&q_builder.location())?;

            start_file.write_all(&q_builder.to_bytes())?;

            term.write_line("Hit Enter when you are ready to try solution out.")?;
            term.read_line()?;

            test_pass = panic::catch_unwind(AssertUnwindSafe(|| {
                try_test.pass(q_builder.location());
            }));
            term.clear_screen()?;
        }
        // remove file name
        dir.pop();

        term.write_line(MSGS[idx])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        start_adventure();
    }
}
