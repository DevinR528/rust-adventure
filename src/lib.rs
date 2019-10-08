//! serialize questions in struct iterate through deserialized for each ?
//!
use std::any::Any;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use console::{style, Term};
use flate2::{write::GzEncoder, Compression};

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
    let f = file.as_ref().file_name().unwrap().to_str().unwrap();
    let file_name = format!("{}.gz", f);
    let mut path = PathBuf::from(file.as_ref());

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    let question = serde_json::to_string(&item)?;
    encoder.write_all(question.as_ref())?;
    encoder.write_all(b"\n")?;
    let gz = encoder.finish()?;

    path.pop();
    path.pop();

    path.push("questions_ser");

    fs::DirBuilder::new().recursive(true).create(&path)?;

    path.push(file_name);
    fs::write(path, gz)?;
    Ok(())
}

pub fn start_adventure() -> io::Result<()> {
    let mut dir = env::current_dir()?;
    dir.push("questions_ser");
    let term = Term::stdout();

    for (idx, q) in QUESTIONS.iter().enumerate() {
        let mut test_pass: Result<(), Box<(dyn Any + Send)>> = Err(Box::new(0));
        // add then file name
        dir.push(format!("{}.gz", q));
        let q_builder = ques::QuestionHolder::new(&dir)?;

        fs::DirBuilder::new().recursive(true).create(
            q_builder
                .folder()
                .expect("DirBuilder failed, no parent in paht"),
        )?;
        let mut start_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&q_builder.location())?;
        start_file.write_all(&q_builder.to_bytes())?;

        while let Err(_) = test_pass {
            let msg = format!(
                "{}",
                style("\nHit Enter when you are ready to try solution out.").cyan()
            );
            term.write_line(&msg)?;
            term.read_line()?;

            test_pass = panic::catch_unwind(AssertUnwindSafe(|| {
                let try_test = trybuild::TestCases::new();
                try_test.pass(q_builder.location());
            }));
        }
        // remove file name and delete so user cant go back while working on current file
        dir.pop();
        fs::remove_file(q_builder.location())?;

        // term.clear_screen()?;

        

        term.write_line(MSGS[idx])?;
    }

    Ok(())
}


