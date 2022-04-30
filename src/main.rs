
use clap::Parser;
// use std::path::Path;

use bootfrost::solver::*;
use bootfrost::strategies::strategies::Strategy;


#[derive(Parser,Default,Debug)]
#[clap(author="Aleksandr Larionov", version, about="Bootfrost Solver")]
struct Arguments{
	#[clap(short, long)]
	/// Path to the file containing the formula
	formula: String,

	#[clap(short, long)]
	/// Strategy: "plain", "general", "manual" or path to the file containing the user strategy
	strategy: String,

	#[clap(short, long)]
	/// Maximum number of steps
	limit: usize,
}


fn main() {

	let args = Arguments::parse();
	println!("{:?}", args);

	let s = match args.strategy.as_str(){
		"plain" => Strategy::PlainShift,
		"general" => Strategy::General,
		"manual" => Strategy::Manual,
		_ => {
			panic!("Invalid strategy name. Type plain, general or manual.");
		},
	};

	let mut solver = Solver::parse_file(&args.formula, s);
	solver.print();
	solver.solver_loop(args.limit);
}
