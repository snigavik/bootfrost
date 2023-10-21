use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StepItem{
	pub step: usize,
	pub qid: usize,
	pub answer: String,
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
		let x = StepItem{step:n, qid:0, answer: "".to_string(), completed: false};
		self.log.push(x);
	}

	// set question and answer
	pub fn set_qa(&mut self, q: usize, a: String){
		let mut x = self.log.last_mut().unwrap();
		x.qid = q;
		x.answer = a;
	}
}