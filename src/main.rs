mod misc;
mod term;
mod answer;
mod context;
mod question;
mod solver;
mod plain;
mod parser;

use crate::parser::*;
use crate::solver::*;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub pcf); // synthesized by LALRPOP
lalrpop_mod!(pub tqfline); // synthesized by LALRPOP

fn main() {
	let mut solver = Solver::parse("./formula.pcf");
	solver.print();
	println!("================");
	solver.solver_loop(100);
}
