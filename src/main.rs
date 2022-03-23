mod misc;
mod term;
mod answer;
mod context;
mod question;
mod solver;
mod plain;
mod parser;

use crate::parser::*;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub pcf); // synthesized by LALRPOP
lalrpop_mod!(pub tqfline); // synthesized by LALRPOP

fn main() {
	parse();
    //let prepcf = crate::pcf::PlainFormulaParser::new().parse("![][A(a)][?[x][A(x)][]]").unwrap();
    //println!("{}", prepcf);
}
