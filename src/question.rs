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

		self.used_answers.retain(|q| q.bid != bid);
	}

	pub fn branches(&self, tqfs: &Vec<Tqf>) -> usize{
		tqfs[self.aformula.0].next.len()
	}

	pub fn last_level(&self, curr_level: usize) -> Option<usize>{
		if self.used_answers.is_empty(){
			None
		}else{
			dbg!(self.used_answers.last().unwrap().level);
			dbg!(curr_level);
			Some(curr_level - self.used_answers.last().unwrap().level.unwrap())
		}
	}

	pub fn used_count(&self) -> usize{
		self.used_answers.len()
	}

	pub fn gs_state(&self, tqfs: &Vec<Tqf>, curr_level: usize) -> (usize, Option<usize>, usize){
		(self.branches(tqfs), self.last_level(curr_level), self.used_count())
	}

	pub fn gs_rate(&self, tqfs: &Vec<Tqf>, curr_level: usize, q_len: usize) -> f64{
		let (_bn, _last, _k) = self.gs_state(tqfs, curr_level);
		
		let bn = _bn as f64;

		let last = if let Some(m) = _last{
			(m as f64)
		}else{
			//1.0/(q_len as f64)	
			0.5
		};

		let qn = q_len as f64;

		let k = _k as f64;
		
		bn * k * (last / qn)
	}
}












//
