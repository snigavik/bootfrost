
pub struct StepItem{
	pub step: usize,
	pub qid: usize,
	pub subst: String
}

pub struct SolverLog{
	pub log: Vec<StepItem>,
	pub curr_step: usize
}

impl SolverLog{
	pub fn new() -> SolverLog{
		SolverLog{log: vec![], curr_step: 0}
	}
}