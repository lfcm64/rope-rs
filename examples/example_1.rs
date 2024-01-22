use rope_rs::rope::Rope;
use ropey;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), std::io::Error> {
    let mut start = Instant::now();

    let path = Path::new("./test_files/large_test.txt");
    let _rope = Rope::new().from_file(path)?;

    println!("{:?}", Instant::now() - start);

    start = Instant::now();

    let _rope2 = ropey::Rope::from_reader(File::open("./test_files/large_test.txt")?)?;

    println!("{:?}", Instant::now() - start);

    Ok(())
}
