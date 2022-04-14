use bootfrost::solver::*;

#[test]
fn test1(){
	let mut solver = Solver::parse_file("./problems/formula.pcf");
	let r = solver.solver_loop(150);
	assert_eq!(SolverResultType::Refuted, r.t);
}


#[test]
fn test2(){
	let mut solver = Solver::parse_file("./problems/solverfunction.pcf");
	let r = solver.solver_loop(150);
	assert_eq!(SolverResultType::Refuted, r.t);
}