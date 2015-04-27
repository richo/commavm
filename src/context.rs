use std::io;
use analysis;

pub struct Func {
    pub name: String,
    pub locals: Vec<analysis::Name>,
    pub stmts: Vec<analysis::Stmt>,
}

pub struct Ctx {
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

    pub fn emit<W: io::Write>(&self, writer: &mut W) -> Result<usize, io::Error> {
        let mut acc: usize = 0;
        for f in self.fns() {
            acc += try!(writer.write(f.name.as_bytes()));
            acc += try!(writer.write("() {\n".as_bytes()));

            acc += try!(writer.write("# Locals: ".as_bytes()));
            acc += try!(writer.write(f.locals.connect(", ").as_bytes()));
            acc += try!(writer.write("\n".as_bytes()));


            acc += try!(writer.write("# Stmts:\n".as_bytes()));
            for s in &f.stmts {
                acc += try!(writer.write(format!("# {:?}\n", s).as_bytes()));
            }

            acc += try!(writer.write("}\n\n".as_bytes()));
            try!(writer.flush());
        }
        Ok(acc)
    }

    fn declare_local(&mut self, name: &str) {
    }
}

