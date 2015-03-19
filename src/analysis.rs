use syntax::{parse,ast,abi};
use std::path;
use std::io::Write;
use std::io;
use std::collections::HashMap;

pub type Name = String;

pub fn locals(blk: &ast::Block) -> Vec<Name> {
    let mut locals: Vec<Name> = Vec::new();
    for stmt in &blk.stmts {
        // TODO Other types of decl
        if let ast::StmtDecl(ref decl, ref id) = stmt.node {
            if let ast::DeclLocal(ref n) = decl.node {
                if let ast::PatIdent(ref mode, ref span, ref pat) = n.pat.node {
                    locals.push(span.node.name.as_str().to_string());
                }
            }
        }
    }

    locals
}
