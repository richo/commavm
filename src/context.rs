use std::io::Write;
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

    pub fn emit<W: Write>(&self, writer: &mut W) {
        for f in self.fns() {
            writer.write(f.name.as_bytes());
            writer.write("() {\n".as_bytes());

            writer.write("# Locals: ".as_bytes());
            writer.write(f.locals.connect(", ").as_bytes());
            writer.write("\n".as_bytes());


            writer.write("# Stmts:\n".as_bytes());
            for s in &f.stmts {
                writer.write(format!("# {:?}\n", s).as_bytes());
            }

            writer.write("}\n\n".as_bytes());
            writer.flush();
        }
    }

    fn declare_local(&mut self, name: &str) {
    }
}

