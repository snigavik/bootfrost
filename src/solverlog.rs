use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StepItem{
	pub step: usize,
	pub question: usize,
	pub answer: String,
	pub atoms_added: String,
	pub atoms_used: String,
	pub base: String,
	//pub completed: bool   
}

#[derive(Serialize)]
pub struct SolverLog{
	pub log: Vec<StepItem>,
	//pub curr_step: usize
}

impl SolverLog{
	pub fn new() -> SolverLog{
		SolverLog{
			log: vec![], 
			//curr_step: 0
		}
	}

	pub fn is_empty(&self) -> bool{
		self.log.is_empty()
	}

	pub fn new_step(&mut self, n: usize){
		let x = StepItem{
			step:n, 
			question:0, 
			answer: "".to_string(), 
			atoms_added: "".to_string(),
			atoms_used: "".to_string(),
			base: "".to_string(),
			//completed: false
		};
		self.log.push(x);
	}

	// set question and answer
	pub fn set_qa(&mut self, q: usize, a: String){
		let mut x = self.log.last_mut().unwrap();
		x.question = q;
		x.answer = a;
	}

	pub fn set_atoms(&mut self, a_a: String, a_u: String){
		let mut x = self.log.last_mut().unwrap();
		x.atoms_added = a_a;
		x.atoms_used = a_u;		
	}

	pub fn set_base(&mut self, b: String){
		let mut x = self.log.last_mut().unwrap();
		x.base = b;		
	}
}


