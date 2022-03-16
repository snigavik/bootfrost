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
	bid: BlockId,
	pub aformula: TqfId,
	pub fstack_i:usize,
	pub curr_answer_stack: Vec<Answer>,
	pub answers: Vec<Answer>,
}












//
