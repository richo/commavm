#![feature(rustc_private)]

extern crate syntax;

use syntax::{parse,ast,abi};
use std::path;

fn main() {
    let ref input = ::std::os::args()[1];
    let file = path::Path::new(input);


    let sess = parse::new_parse_sess();
    let cfg = vec![];

    let mut parser = parse::new_parser_from_file(&sess, cfg, &file);

    let krate = parser.parse_crate_mod();
    println!("{:?}", krate);
}
