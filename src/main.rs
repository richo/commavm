#![feature(rustc_private)]

extern crate syntax;

use syntax::{parse,ast,abi};
use std::path;
use std::io::Write;
use std::io;
use std::collections::HashMap;

mod analysis;

struct Func {
    pub name: String,
    locals: Vec<analysis::Name>,
    stmts: Vec<analysis::Stmt>,
}

enum Value {
    Declared,
    Literal(String),
}

struct Ctx {
    fns: Vec<Func>,
}

impl Ctx {
    pub fn new() -> Ctx {
        Ctx {
            fns: vec![],
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

            writer.write("# Locals: ".as_bytes());
            writer.write(f.locals.connect(", ").as_bytes());

            writer.write("# Stmts:\n".as_bytes());
            for s in &f.stmts {
                writer.write(format!("# {:?}\n", s).as_bytes());
            }

            writer.write("\n}\n".as_bytes());
            writer.flush();
        }
    }

    fn declare_local(&mut self, name: &str) {
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
                let vars = analysis::locals(blk);
                let stmts = analysis::stmts(blk);

                ctx.add_fn(Func {
                    name: it.ident.as_str().to_string(),
                    locals: vars,
                    stmts: stmts,
                });
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
