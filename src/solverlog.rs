use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StepItem{
	pub step: usize,
	pub qid: usize,
	pub subst: String,
	pub completed: bool   
}

#[derive(Serialize)]
pub struct SolverLog{
	pub log: Vec<StepItem>,
	pub curr_step: usize
}

impl SolverLog{
	pub fn new() -> SolverLog{
		SolverLog{log: vec![], curr_step: 0}
	}

	pub fn new_step(&mut self, n: usize){
		let x = StepItem{step:n, qid:0, subst: "".to_string(), completed: false};
		self.log.push(x);
	}
}