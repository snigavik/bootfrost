
use bootfrost::solver::*;
use bootfrost::strategies::strategies::Strategy;

fn main() {
	let mut solver = Solver::parse_file("./problems/branch1.pcf", Strategy::Manual);
	solver.print();
	println!("================");
	solver.solver_loop(50);
}
