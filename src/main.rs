
use bootfrost::solver::*;
use bootfrost::strategies::strategies::Strategy;

fn main() {
	let mut solver = Solver::parse_file("./problems/remove.pcf", Strategy::General);
	solver.print();
	solver.solver_loop(50);
}
