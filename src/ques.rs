use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct QuestionHolder {
    question: String,
    ans_loc: PathBuf,
}

impl QuestionHolder {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = fs::File::open(path.as_ref())?;
        let mut decoder = GzDecoder::new(file);
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed)?;

        let this = serde_json::from_str(&decompressed)?;
        Ok(this)
    }
    pub(crate) fn build<P: AsRef<Path>>(question: String, ans_loc: P) -> Self {
        let file_name = ans_loc.as_ref().file_name().expect("build QuestionHolder failed no file_name");
        let mut ans_loc = PathBuf::from(ans_loc.as_ref());
        // remove file name then question folder
        ans_loc.pop();
        ans_loc.pop();
        // push answers folder and same file name
        ans_loc.push("answers/");
        ans_loc.push(file_name);

        Self { question, ans_loc }
    }
    pub(crate) unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.question.as_bytes_mut()
    }

    pub(crate) fn folder(&self) -> Option<&Path> {
        self.ans_loc.parent()
    }

    pub(crate) fn location(&self) -> &Path {
        self.ans_loc.as_ref()
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        self.question.bytes().collect::<Vec<_>>()
    }
}
