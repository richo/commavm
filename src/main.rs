#![feature(rustc_private)]

extern crate syntax;

use syntax::{parse,ast,abi};
use std::path;
use std::io::Write;
use std::io;
use std::collections::HashMap;

struct Func {
    pub name: String,
}

enum Value {
    Literal(String),
}


struct Ctx {
    fns: Vec<Func>,
    locals: HashMap<String, Value>,
}

impl Ctx {
    pub fn new() -> Ctx {
        Ctx {
            fns: vec![],
            locals: HashMap::new(),
        }
    }


    pub fn fns(&self) -> &Vec<Func>{
        &self.fns
    }

    pub fn add_fn(&mut self, f: Func) {
        self.fns.push(f);
    }
}

impl Ctx {
    fn emit<W: Write>(&self, writer: &mut W) {
        for f in self.fns() {
            writer.write(f.name.as_bytes());
            writer.write("() {\n".as_bytes());
            // TODO Write out body
            writer.write("}\n".as_bytes());
            writer.flush();
        }
    }
}

fn load_file(name: &str) -> ast::Crate {
    let file = path::Path::new(name);


    let sess = parse::new_parse_sess();
    let cfg = vec![];

    let mut parser = parse::new_parser_from_file(&sess, cfg, &file);

    parser.parse_crate_mod()
}

fn process_crate(krate: ast::Crate) -> Ctx {
    let mut ctx = Ctx::new();

    for it in &krate.module.items {
        match it.node {
            ast::ItemFn(ref dec, safety, abi, _, ref blk) => {
                ctx.add_fn(Func { name: it.ident.as_str().to_string() });
            },
            ref other => {
                panic!("Unexpected Item: {:?}", other);
            },
        }
    }

    ctx
}

fn main() {
    let ref input = ::std::os::args()[1];

    let krate = load_file(input);
    let ctx = process_crate(krate);

    ctx.emit(&mut io::stdout());
}
