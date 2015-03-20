use syntax::{parse,ast,abi};
use std::path;
use std::io::Write;
use std::io;
use std::collections::HashMap;

pub type Name = String;

pub fn locals(blk: &ast::Block) -> Vec<Name> {
    let mut locals = Vec::new();
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

// Do something more gracefull with a state table, since the giant global namespace thing is
// probably not super sustainable
pub type Receiver = String;

// Opaque for now
#[derive(Clone)]
struct Value;

#[derive(Clone)]
enum Arg {
    Literal(Value),
    Bound(Name),
}

struct Funcall {
    receiver: Receiver,
    arguments: Vec<Arg>
}

struct Assignment {
    receiver: Name,
    target: Name, // TODO
}

pub enum Stmt {
    Funcall(Funcall),
    Assignment(Assignment),
}

impl Stmt {
    // Mostly just for sanity reasons
    fn funcall(receiver: ast::Ident, arguments: &[Arg]) -> Stmt {
        Stmt::Funcall(Funcall {
            receiver: receiver.as_str().to_string(),
            arguments: arguments.to_vec(),
        })
    }
}

pub fn stmts(blk: &ast::Block) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    for stmt in &blk.stmts {
        match stmt.node {
            ast::StmtDecl(..) => {},
            ast::StmtSemi(ref expr, ref id) => {
                if let ast::ExprCall(ref expr, ref id) = expr.node {
                    if let ast::ExprPath(_, ref path) = expr.node {
                        let ref segment = path.segments[0];
                        stmts.push(Stmt::funcall(segment.identifier, &[]));
                    }
                } else
                if let ast::ExprAssign(ref expr, ref id) = expr.node {
                    if let ast::ExprPath(_, ref path) = expr.node {
                        // println!("Assign: {:?}", path.segments);
                    }
                }
            },
            _ => {
                println!("unhandled {:?}", stmt);
            }
        }
    }

    stmts
}
