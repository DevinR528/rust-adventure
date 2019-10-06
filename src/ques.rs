use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
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
    pub(crate) fn build(question: String, ans_loc: PathBuf) -> Self {
        Self { question, ans_loc, }
    }
    pub(crate) unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.question.as_bytes_mut()
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        self.question.bytes().collect::<Vec<_>>()
    }
}
