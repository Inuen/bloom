pub mod bloom;

fn main() {
    let mut bloom = bloom::Bloom::default();
    bloom.insert_item("asd");
    bloom.insert_item("sad");
    bloom.insert_item("TO");
    bloom.insert_item("TOF");
    bloom.insert_item("fOOT");
    let mc = bloom.might_contain("TOFU");
    assert!(!mc);
}
