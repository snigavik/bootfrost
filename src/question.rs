use crate::misc::*;
use crate::answer::*;
use crate::solver::*;

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

		let k = _k as f64 + 0.5;
		
		bn * k * (last / qn)
	}


	// ----
	fn find_answer_local(&mut self, si: &StrategyItem, bid: BlockId, tqfs: &Vec<Tqfs>, base: &Base) -> Option<Answer>{
		let limit = si.limit;
		if let Some(top) = self.curr_answer_stack.last(){
			if top.bid != bid{
				let mut new_top = top.clone();
				new_top.bid = bid;
				self.curr_answer_stack.push(new_top);
			}
		}else{
			let new_top = Answer::new(bid, qid, base.len(), tqfs[aformula.0].conj.len());
			self.curr_answer_stack.push(new_top);
		}


		let mut i = 0;
		while i < limit{
			let a = &self.curr_answer_stack.last().unwrap();
			i = i + 1;
			match &self.curr_answer_stack.last_mut().unwrap().state{
				MatchingState::Success | MatchingState::NextA | MatchingState::Zero => {
					self.next_a(qid);
					continue;
				},
				MatchingState::NextB | MatchingState::Fail => {
					self.next_b(qid);
					continue;
				},
				MatchingState::Ready => {
					let context = &self.stack[self.questions[qid.0].fstack_i].context;
					match self.curr_answer_stack.last().unwrap().last().unwrap(){
						LogItem::Matching{batom_i, qatom_i, ..} => {
							let bterm = &self.base[*batom_i];
							if bterm.deleted{
								self.curr_answer_stack.last_mut().unwrap().state = MatchingState::Fail;
								continue;
							}
							let btid = bterm.term;
							let qtid = tqfs[aformula.0].conj[*qatom_i];
							
							let mut curr_answer = &mut self.curr_answer_stack.last_mut().unwrap();
							if matching(btid, qtid, context, curr_answer, self){
								self.curr_answer_stack.last_mut().unwrap().state = MatchingState::Success;
								continue;
							}else{
								self.curr_answer_stack.last_mut().unwrap().state = MatchingState::Fail;
								continue;								
							}
						},
						LogItem::Interpretation{qatom_i} => {
														
							let qtid = tqfs[aformula.0].conj[*qatom_i];
							let curr_answer = &self.curr_answer_stack.last_mut().unwrap();
							let b = processing(qtid, context, Some(&curr_answer), self).unwrap();
							if self.psterms.check_value(&b){
								self.curr_answer_stack.last_mut().unwrap().state = MatchingState::Success;
							}else{
								self.curr_answer_stack.last_mut().unwrap().state = MatchingState::Fail;
							}
							continue;
						},
					}
				},
				MatchingState::Rollback => {
					if self.curr_answer_stack.last_mut().unwrap().len() > 0{
						self.curr_answer_stack.last_mut().unwrap().state = MatchingState::NextB;
						continue;
					}else{
						self.curr_answer_stack.last_mut().unwrap().state = MatchingState::NextK;
						continue;
					}
				},
				MatchingState::NextK => {
					self.next_k(qid);
				},
				MatchingState::Exhausted => {
					if self.next_bounds(qid){
						self.curr_answer_stack.last_mut().unwrap().state = MatchingState::Zero;
					}else{
						break;
					}
				},
				MatchingState::Answer => {

					if self.curr_answer_stack.last_mut().unwrap().conj_len == 0{
						self.curr_answer_stack.last_mut().unwrap().state = MatchingState::Empty;	
					}else{
						self.curr_answer_stack.last_mut().unwrap().state = MatchingState::NextB;
					}
					

					let nq =self.curr_answer_stack.last_mut().unwrap().clone();
					if let Some(_) = self.answers.iter().find(|x| *x == &nq){
						continue;
					}					
					self.answers.push(nq);

					let mut answer1 = self.answers.last().unwrap().clone();
					match si.selector{
						SelectorStrategy::First(f) => {
							if f(&answer1, &self.psterms){
								answer1.level = Some(self.stack.iter().filter(|x| x.activated).count());
								self.used_answers.push(answer1.clone());
								return Some(answer1)
							}else{
								continue;
							}
						},
						SelectorStrategy::Best => {
							continue;
						}
					}
				},
				MatchingState::Empty => {
					break;
				}
			}

		}
		None //		 
	}	
}












//
