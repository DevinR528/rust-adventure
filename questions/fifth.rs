#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerialDep<'a> {
    name: &'a str,
    version: Version,
}
impl<'a> SerialDep<'a> {
    fn new<'b: 'a>(key: CrateKey, uni: &'b Universe) -> SerialDep<'a> {
        let version = uni.crates[&key.name][key.index as usize].num.clone();
        Self { name: &key.name, version, }
    }
}
