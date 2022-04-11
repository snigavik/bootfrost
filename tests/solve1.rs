use bootfrost::solver::*;

#[test]
fn test1(){
	let mut solver = Solver::parse("./problems/formula.pcf");
	solver.solver_loop(150);
}