use crate::misc::*;
use crate::answer::*;

#[derive(Clone)]
pub enum Quantifier{
    Forall,
    Exists,
}

pub struct Tqf{
	quantifier: Quantifier,
	vars: Vec<TermId>,
	pub conj: Vec<TermId>,
	commands: Vec<TermId>,
	next: Vec<TqfId>,
}

impl Tqf{
	pub fn conj_len(&self) -> usize{
		self.conj.len()
	}
}


pub struct Question{
	pub aformula: TqfId,
	bid: BlockId,
	pub answer_state: AnswerState,
	// curr_answer: Answer,
	// bounds: Bounds,
}












//
