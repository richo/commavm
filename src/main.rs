#![feature(rustc_private)]

extern crate syntax;

use syntax::{parse,ast,abi};
use std::path;
use std::old_io::stdio;

struct Func {
    pub name: String,
}

struct Ctx {
    fns: Vec<Func>,
}

impl Ctx {
    pub fn new() -> Ctx {
        Ctx { fns: vec![] }
    }


    pub fn fns(&self) -> &Vec<Func>{
        &self.fns
    }

    pub fn add_fn(&mut self, f: Func) {
        self.fns.push(f);
    }
}

fn emit(ctx: Ctx) {
    for f in ctx.fns() {
        print!("{}()", f.name);
        stdio::println("{\n}");
    }
}

fn main() {
    let ref input = ::std::os::args()[1];
    let file = path::Path::new(input);


    let sess = parse::new_parse_sess();
    let cfg = vec![];

    let mut parser = parse::new_parser_from_file(&sess, cfg, &file);

    let krate = parser.parse_crate_mod();

    let mut ctx = Ctx::new();

    for it in &krate.module.items {
        match it.node {
            ast::ItemFn(ref dec, safety, abi, _, ref blk) => {
                ctx.add_fn(Func { name: it.ident.as_str().to_string() });
            },
            _ => {},
        }
    }

    // Jank
    emit(ctx);
}
