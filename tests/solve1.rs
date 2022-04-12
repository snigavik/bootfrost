use bootfrost::solver::*;

#[test]
fn test1(){
	let mut solver = Solver::parse("./problems/formula.pcf");
	let r = solver.solver_loop(150);
	assert_eq!(SolverResultType::Refuted, r.t);
}