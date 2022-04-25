
use bootfrost::solver::*;


fn main() {
	let mut solver = Solver::parse_file("./problems/remove.pcf");
	solver.print();
	println!("================");
	solver.solver_loop(50);
}
