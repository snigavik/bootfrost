use crate::misc::*;
use crate::answer::*;

pub struct Tqf{
	pub quantifier: Quantifier,
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
	pub fstack_i:usize, // position in the stack where we can find corresponding context
	pub curr_answer_stack: Vec<Answer>,
	pub answers: Vec<Answer>,
	pub used_answers: Vec<Answer>,
}

impl Question{
	pub fn remove_answers(&mut self, bid:BlockId){
		self.answers.retain(|q| q.bid != bid);
		self.curr_answer_stack.retain(|q| q.bid != bid);
	}

	pub fn branches(&self, tqfs: &Vec<Tqf>) -> usize{
		tqfs[self.aformula.0].next.len()
	}

	pub fn last_level(&self, curr_level: usize) -> Option<usize>{
		if self.used_answers.is_empty(){
			None
		}else{
			Some(curr_level - self.used_answers.last().unwrap().level.unwrap())
		}
	}

	pub fn used_count(&self) -> usize{
		self.used_answers.len()
	}

	pub fn gs_state(&self, tqfs: &Vec<Tqf>, curr_level: usize) -> (usize, Option<usize>, usize){
		(self.branches(tqfs), self.last_level(curr_level), self.used_count())
	}
}












//
