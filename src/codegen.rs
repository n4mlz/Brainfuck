use std::fmt::Write as _;

use crate::parser::Node;

const TAPE_LEN: i64 = 30_000;

pub fn generate_ir(nodes: &[Node]) -> String {
    let mut cg = Codegen::new();
    cg.preamble();
    cg.main_prelude();
    cg.emit_nodes(nodes);
    cg.line("ret i32 0");
    cg.end_main();
    cg.finish()
}

struct Codegen {
    out: String,
    indent: usize,
    uniq: usize,
}

impl Codegen {
    fn new() -> Self {
        Self {
            out: String::with_capacity(16 * 1024),
            indent: 0,
            uniq: 0,
        }
    }

    fn finish(self) -> String {
        self.out
    }

    fn preamble(&mut self) {
        writeln!(
            self.out,
            "@tape = internal global [{TAPE_LEN} x i8] zeroinitializer"
        )
        .unwrap();
        writeln!(self.out, "declare i32 @putchar(i32)").unwrap();
        writeln!(self.out, "declare i32 @getchar()").unwrap();
        self.out.push('\n');
    }

    fn main_prelude(&mut self) {
        writeln!(self.out, "define i32 @main() {{").unwrap();
        self.indent = 1;
        self.label("entry");
        self.line("%ptr = alloca i8*");
        self.line(&format!(
            "%base = getelementptr [{TAPE_LEN} x i8], [{TAPE_LEN} x i8]* @tape, i64 0, i64 0"
        ));
        self.line("store i8* %base, i8** %ptr");
    }

    fn end_main(&mut self) {
        self.indent = 0;
        writeln!(self.out, "}}").unwrap();
    }

    fn label(&mut self, name: &str) {
        writeln!(self.out, "{name}:").unwrap();
    }

    fn line(&mut self, s: &str) {
        for _ in 0..self.indent {
            self.out.push_str("  ");
        }
        writeln!(self.out, "{s}").unwrap();
    }

    fn fresh(&mut self, prefix: &str) -> String {
        let id = self.uniq;
        self.uniq += 1;
        format!("%{prefix}{id}")
    }

    pub fn emit_nodes(&mut self, nodes: &[Node]) {
        for n in nodes {
            self.emit_node(n);
        }
    }

    fn emit_node(&mut self, n: &Node) {
        match n {
            Node::IncPtr => self.emit_move_ptr(1),
            Node::DecPtr => self.emit_move_ptr(-1),
            Node::IncCell => self.emit_add_cell(1),
            Node::DecCell => self.emit_add_cell(-1),
            Node::Output => self.emit_output(),
            Node::Input => self.emit_input(),
            Node::Loop(body) => self.emit_loop(body),
        }
    }

    fn emit_move_ptr(&mut self, delta: i64) {
        let p = self.fresh("p");
        let q = self.fresh("q");
        self.line(&format!("{p} = load i8*, i8** %ptr"));
        self.line(&format!("{q} = getelementptr i8, i8* {p}, i64 {delta}"));
        self.line(&format!("store i8* {q}, i8** %ptr"));
    }

    fn emit_add_cell(&mut self, delta: i64) {
        let p = self.fresh("p");
        let v0 = self.fresh("v");
        let v1 = self.fresh("v");
        self.line(&format!("{p} = load i8*, i8** %ptr"));
        self.line(&format!("{v0} = load i8, i8* {p}"));
        if delta >= 0 {
            self.line(&format!("{v1} = add i8 {v0}, {delta}"));
        } else {
            self.line(&format!("{v1} = sub i8 {v0}, {}", -delta));
        }
        self.line(&format!("store i8 {v1}, i8* {p}"));
    }

    fn emit_output(&mut self) {
        let p = self.fresh("p");
        let v = self.fresh("v");
        let w = self.fresh("w");
        self.line(&format!("{p} = load i8*, i8** %ptr"));
        self.line(&format!("{v} = load i8, i8* {p}"));
        self.line(&format!("{w} = zext i8 {v} to i32"));
        self.line(&format!("call i32 @putchar(i32 {w})"));
    }

    fn emit_input(&mut self) {
        let p = self.fresh("p");
        let c = self.fresh("c");
        let eof = self.fresh("eof");
        let cz = self.fresh("cz");
        let b = self.fresh("b");
        self.line(&format!("{c} = call i32 @getchar()"));
        self.line(&format!("{eof} = icmp slt i32 {c}, 0"));
        self.line(&format!("{cz} = select i1 {eof}, i32 0, i32 {c}"));
        self.line(&format!("{b} = trunc i32 {cz} to i8"));
        self.line(&format!("{p} = load i8*, i8** %ptr"));
        self.line(&format!("store i8 {b}, i8* {p}"));
    }

    fn emit_loop(&mut self, body: &[Node]) {
        let id = self.uniq;
        self.uniq += 1;

        let l_cond = format!("loop.cond.{id}");
        let l_body = format!("loop.body.{id}");
        let l_end = format!("loop.end.{id}");

        self.line(&format!("br label %{l_cond}"));
        self.label(&l_cond);

        let p = self.fresh("p");
        let v = self.fresh("v");
        let nz = self.fresh("nz");
        self.indent = 1;
        self.line(&format!("{p} = load i8*, i8** %ptr"));
        self.line(&format!("{v} = load i8, i8* {p}"));
        self.line(&format!("{nz} = icmp ne i8 {v}, 0"));
        self.line(&format!("br i1 {nz}, label %{l_body}, label %{l_end}"));

        self.label(&l_body);
        self.indent = 1;
        self.emit_nodes(body);
        self.line(&format!("br label %{l_cond}"));

        self.label(&l_end);
        self.indent = 1;
    }
}
