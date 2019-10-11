// When implementing the `Iterator` trait you return
// a reference of each element from the original data.
// Here `Bytes` is just a wrapper so there are easier ways to
// impl `Iterator` but for our lifetime adventure this works fine.
// We need to make sure `BytesIter` and each ref yielded
// only live for the duration of the call to `iter`.

struct Bytes(Vec<u8>);
impl Bytes {
    fn iter(&self) -> BytesIter {
        BytesIter { bytes: self, idx: 0, }
    }
}

struct BytesIter {
    bytes: &Bytes,
    idx: usize,
}
impl Iterator for BytesIter {

    type Item = &u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.bytes.0.get(self.idx - 1)
    }
}

fn main() {}

