// mod misc;
// mod term;
// mod answer;
// mod context;
// mod question;
// mod solver;
// mod plain;
// mod parser;
// mod ifunctions;
// mod strategies;
// mod base;
// mod environment;

// use crate::parser::*;
// use crate::solver::*;

// extern crate bootfrost;
// use bootfrost::parser::*;
use bootfrost::solver::*;

// #[macro_use] extern crate lalrpop_util;
// lalrpop_mod!(pub pcf); // synthesized by LALRPOP
// lalrpop_mod!(pub tqfline); // synthesized by LALRPOP

fn main() {
	let mut solver = Solver::parse_file("./problems/solverfunction.pcf");
	solver.print();
	println!("================");
	solver.solver_loop(150);
}
