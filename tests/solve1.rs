use bootfrost::solver::*;

#[test]
fn test(){
	let mut solver = Solver::parse("./problems/formula.pcf");
	solver.solver_loop(150);
}