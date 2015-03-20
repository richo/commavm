#![feature(rustc_private)]

extern crate syntax;

use syntax::{parse,ast,abi};
use std::path;
use std::io;
use std::collections::HashMap;
use context::{Ctx,Func};

mod analysis;
mod coherence;
mod context;

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
