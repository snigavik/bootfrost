mod misc;
mod term;
mod answer;
mod context;
mod question;
mod solver;
mod plain;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub pcf); // synthesized by LALRPOP

fn main() {
    let prepcf = crate::pcf::PlainFormulaParser::new().parse("![][A(a)][?[x][A(x)][]]").unwrap();
    //println!("{}", prepcf);
}
