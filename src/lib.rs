//! serialize questions in struct iterate through deserialized for each ?
//!
use std::any::Any;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use console::{style, Term};
use flate2::{write::GzEncoder, Compression, read::GzDecoder};

use trybuild;

const QUESTIONS: &[&str] = &["first.rs", "second.rs", "third.rs", "fourth.rs"];
const MSGS: &[&str] = &[
    "Great job, on to invariance, covariance and contravariance!! just kidding",
    "Wow you're doing great!!",
    "Now you're ready for a pull request to the borrow checker!!",
    "Great keep up the good work."
];

pub fn serialize_all_questions() -> io::Result<()> {
    let mut dir = env::current_dir()?;
    
    dir.push("questions");

    for file in QUESTIONS.iter() {
        dir.push(file);
        let code = fs::read_to_string(&dir)?;
        write_new_question(&dir, &code)?;
        dir.pop();
    }
    Ok(())
}

fn write_new_question<P: AsRef<Path>>(file: P, item: &str) -> io::Result<()> {
    let f = file.as_ref().file_name().unwrap().to_str().unwrap();
    let file_name = format!("{}.gz", f);
    let mut path = PathBuf::from(file.as_ref());

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(item.as_bytes())?;
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

fn unzip<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let file = fs::File::open(path.as_ref()).expect("open zip");
    let mut decoder = GzDecoder::new(file);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;

    Ok(decompressed)
}

pub fn start_adventure() -> io::Result<()> {
    let mut zip_dir = env::current_dir()?;
    let mut ans_dir = env::current_dir()?;

    println!("{:?}", zip_dir);

    zip_dir.push("questions_ser");
    ans_dir.push("answers");

    let term = Term::stdout();

    for (idx, q) in QUESTIONS.iter().enumerate() {
        let mut test_pass: Result<(), Box<(dyn Any + Send)>> = Err(Box::new(0));
        // add then file name
        zip_dir.push(format!("{}.gz", q));
        let question = unzip(&zip_dir)?;

        fs::DirBuilder::new().recursive(true).create(&ans_dir)?;

        ans_dir.push(q);
        match fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open(&ans_dir)
        {
            Ok(mut file) => file.write_all(&question.bytes().collect::<Vec<_>>())?,
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::AlreadyExists => {},
                    _ => return Err(err),
                }
            }
        }
        // remove file name from loop
        ans_dir.pop();
        while let Err(_) = test_pass {
            let msg = format!(
                "{}",
                style("\nEnter the name of the file to test from answer folder.").cyan()
            );
            term.write_line(&msg)?;
            let file = term.read_line()?;
            ans_dir.push(file);

            test_pass = panic::catch_unwind(AssertUnwindSafe(|| {
                let try_test = trybuild::TestCases::new();
                try_test.pass(&ans_dir);
            }));
            // remove user supplied file name
            ans_dir.pop();
        }
        zip_dir.pop();
        // term.clear_screen()?;

        term.write_line(MSGS[idx])?;
    }

    Ok(())
}


