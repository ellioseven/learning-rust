// `std` is the standard library crate.
// `collections` is a module within `std`.
// `LinkedList` is the export from the `collections` module.

mod person;
use std::collections::LinkedList;
use person::greet;

pub fn main() {
    let mut ll = LinkedList::new();

    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(4);

    for foo in ll {
        println!("{}", foo);
    }

    // Some imports are automatically included by `prelude`.
    // eg: std::collections:;Vec
    // @url https://stackoverflow.com/questions/36384840/what-is-the-prelude
    let mut vec = Vec::new();
    vec.push('x');
    vec.push('y');
    vec.push('z');
    for i in vec {
        println!("{}", i);
    }

    // Use function from defined module.
    greet();
}
