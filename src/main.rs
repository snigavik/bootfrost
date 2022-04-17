
use bootfrost::solver::*;


fn main() {
	let mut solver = Solver::parse_file("./problems/observe.pcf");
	solver.print();
	println!("================");
	solver.solver_loop(150);
}
