use bootfrost::solver::*;
use bootfrost::strategies::strategies::Strategy;

#[test]
fn test1(){
	let mut solver = Solver::parse_file("./problems/formula.pcf", Strategy::General);
	let r = solver.solver_loop(150);
	assert_eq!(SolverResultType::Refuted, r.t);
}


#[test]
fn test_subsolver(){
	let mut solver = Solver::parse_file("./problems/solverfunction.pcf", Strategy::General);
	let r = solver.solver_loop(150);
	assert_eq!(SolverResultType::Refuted, r.t);
}

#[test]
fn test_observe(){
	let mut solver = Solver::parse_file("./problems/observe.pcf", Strategy::General);
	let r = solver.solver_loop(150);
	assert_eq!(SolverResultType::Refuted, r.t);
}

#[test]
fn test_branch1(){
	let mut solver = Solver::parse_file("./problems/branch1.pcf", Strategy::General);
	let r = solver.solver_loop(150);
	assert_eq!(SolverResultType::Refuted, r.t);
}

#[test]
fn test_remove(){
	let mut solver = Solver::parse_file("./problems/remove.pcf", Strategy::General);
	let r = solver.solver_loop(150);
	assert_eq!(SolverResultType::Refuted, r.t);
}