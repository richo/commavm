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
#[derive(Clone,Debug)]
struct Value;

#[derive(Clone,Debug)]
enum Arg {
    Literal(Value),
    Bound(Name),
}

#[derive(Debug)]
struct Funcall {
    receiver: Receiver,
    arguments: Vec<Arg>
}

#[derive(Debug)]
struct Assignment {
    receiver: Name,
    target: Rvalue,
}

#[derive(Debug)]
pub enum Stmt {
    Funcall(Funcall),
    Assignment(Assignment),
}

// Not obviously correct, some of these types need cleaning up pretty desperately
#[derive(Debug)]
pub enum Rvalue {
    Literal(Value),
    Call(Funcall),
    Bound(Name),
}

impl Stmt {
    // Mostly just for sanity reasons
    fn funcall(receiver: ast::Ident, arguments: &[Arg]) -> Stmt {
        Stmt::Funcall(Funcall {
            receiver: receiver.as_str().to_string(),
            arguments: arguments.to_vec(),
        })
    }

    fn assign(receiver: ast::Ident, target: Rvalue) -> Stmt {
        Stmt::Assignment(Assignment {
            receiver: receiver.as_str().to_string(),
            target: target,
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
                        assert!(path.segments.len() == 1);
                        let ref segment = path.segments[0];
                        stmts.push(Stmt::funcall(segment.identifier, &[]));
                    }
                } else
                if let ast::ExprAssign(ref expr, ref id) = expr.node {
                    if let ast::ExprPath(_, ref path) = expr.node {
                        let ref segment = path.segments[0];
                        stmts.push(Stmt::assign(segment.identifier, Rvalue::Literal(Value))); // TODO
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
