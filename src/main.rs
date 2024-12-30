use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

mod macros;

fn main() {

    let world = "Hello world!";
    println!("world: {}", S!(world));

    // // println!("Hello, world!");
    //
    // let f1 = FileInfo::new("/temp/foo");
    // let f2 = FileInfo::new("/temp/bar");
    //
    // // ------------------------------------------------------------------------------
    // // Demonstrate PartialEq. It gives us `==` and `!=`.
    // if f1 == f2 {
    //     println!("f1 and f2 are equal");
    // } else {
    //     println!("f1 and f2 are NOT equal");
    // }
    //
    // if f1 != f2 {
    //     println!("f1 and f2 are NOT equal");
    // } else {
    //     println!("f1 and f2 are equal");
    // }
    //
    // // ------------------------------------------------------------------------------
    // // Demonstrate Hash. Note that the HashMap takes ownership of its keys -
    // // they are moved into the HashMap.
    // let mut hm = HashMap::new();
    // hm.insert(f1, 1);
    // hm.insert(f2, 2);
    //
    // let f_lookup = FileInfo::new("/temp/foo");
    // let file_size = hm[&f_lookup];
    // println!("f1 has a size of {} bytes", file_size);
    //
    // // ------------------------------------------------------------------------------
    // // Demonstrate PartialOrd. It gives us `<`, `<=`, `>=` and `>`.
    //
    // // Makes some new f's because the others went into the HashMap.
    // // Makes some new f's because the others went into the HashMap.
    // let f1 = FileInfo::new("/temp/foo");
    // let f2 = FileInfo::new("/temp/bar");
    //
    // if f1 < f2 {
    //     println!("f1 is less than f2");
    // } else {
    //     println!("f1 is not less than f2");
    // }
    //
    // if f1 > f2 {
    //     println!("f1 is greater than f2");
    // } else {
    //     println!("f1 is not greater than f2");
    // }
    //
    // // ------------------------------------------------------------------------------
    // // Demonstrate Ord. It unlocks sorting functionality.
    // let mut v = vec![f1, f2];
    // v.sort();
    // println!("v after sorting = {:#?}", v);
    //
    // // ------------------------------------------------------------------------------
    // // Demonstrate cross-type equality testing.
    // let f1 = FileInfo::new("/temp/foo");
    // if f1 == "/temp/foo" {
    //     println!("The path in f1 is equal to the str value \"/temp/foo\"");
    // } else {
    //     println!("Nope, comparisons to strings are not working as they should be.");
    // }
}

#[derive(Debug, Default, Clone, Eq)]
struct FileInfo {
    path: PathBuf,
    contents: String,
    is_valid_utf8: bool,
}

impl FileInfo {
    fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }
}

impl PartialEq for FileInfo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialEq<&str> for FileInfo {
    fn eq(&self, other: &&str) -> bool {
        match self.path.to_str() {
            Some(path) => path == *other,
            None => false,
        }
    }
}
