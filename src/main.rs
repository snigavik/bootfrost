use std::mem;
use clap::Parser;

use bootfrost::solver::*;
use bootfrost::strategies::strategies::Strategy;

//use bootfrost::solverlog::*;

#[derive(Parser,Default,Debug)]
#[clap(author="Aleksandr Larionov", version, about="Bootfrost Solver")]
struct Arguments{
	#[clap(short, long)]
	/// Path to the file containing the formula
	formula: String,

	#[clap(short, long)]
	/// Strategy: "plain", "general", "manualfirst", "manualbest" or path to the file containing the user strategy
	strategy: String,

	#[clap(short, long)]
	/// Maximum number of steps
	limit: usize,

	#[clap(short, long)]
	/// json logging
	json: bool,
}


fn main() {

	let args = Arguments::parse();
	println!("{:?}", args);

	let s = match args.strategy.as_str(){
		"plain" => Strategy::PlainShift,
		"general" => Strategy::General,
		"manualfirst" => Strategy::ManualFirst,
		"manualbest" => Strategy::ManualBest,
		_ => {
			panic!("Invalid strategy name. Type plain, general, manualfirst or manualbest.");
		},
	};

	let mut solver = Solver::parse_file(&args.formula, s);
	solver.print();
	solver.solver_loop(args.limit);
	if args.json{
		let j = serde_json::to_string_pretty(&solver.slog).unwrap();
		println!("\n\n---- JSON LOG ----\n {}", j);
	}

}
