use crate::misc::*;
use crate::answer::*;

#[derive(Clone)]
pub enum Quantifier{
    Forall,
    Exists,
}

pub struct Tqf{
	quantifier: Quantifier,
	pub vars: Vec<TermId>,
	pub conj: Vec<TermId>,
	pub commands: Vec<TermId>,
	pub next: Vec<TqfId>,
}

impl Tqf{
	pub fn conj_len(&self) -> usize{
		self.conj.len()
	}
}


pub struct Question{
	pub bid: BlockId,
	pub aformula: TqfId,
	pub fstack_i:usize,
	pub curr_answer_stack: Vec<Answer>,
	pub answers: Vec<Answer>,
}

impl Question{
	pub fn remove_answers(&mut self, bid:BlockId){
		self.answers.retain(|q| q.bid != bid);
		self.curr_answer_stack.retain(|q| q.bid != bid);
	}
}












//
