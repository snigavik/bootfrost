
use bootfrost::solver::*;


fn main() {
	let mut solver = Solver::parse_file("./problems/formula.pcf");
	solver.print();
	println!("================");
	solver.solver_loop(50);
}
