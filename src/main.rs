use ::rust_adventure::{serialize_all_questions, start_adventure};

fn main() {
    // serialize_all_questions().expect("question serialization failed");

    start_adventure().expect("there may be a bug file an issue");
}
