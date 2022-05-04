use serde::{Deserialize, Serialize};

use crate::strategies::strategies::StrategyItem;
use crate::strategies::strategies::SelectorStrategy;
use crate::strategies::attributes::*;
use crate::strategies::environment::PEnv;
use crate::misc::*;
use crate::answer::*;
use crate::base::*;
use crate::term::*;

use crate::solver::*;

use crate::context::*;

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Question{
	pub qid: QuestionId,
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
			m as f64
		}else{
			//1.0/(q_len as f64)	
			0.5
		};

		let qn = q_len as f64;

		let k = _k as f64 + 0.5;
		
		bn * k * (last / qn)
	}



	pub fn find_answer_local(
		&mut self, 
		si: &StrategyItem, 
		bid: BlockId, 
		psterms: &mut PSTerms, 
		tqfs: &Vec<Tqf>, 
		base: &mut Base,
		level: usize,
		context: &Context,
		attributes: &mut Attributes) -> Option<AnswerId>{	

		let limit = si.limit;
		if let Some(top) = self.curr_answer_stack.last(){
			if top.bid != bid{
				let mut new_top = top.clone();
				new_top.bid = bid;
				self.curr_answer_stack.push(new_top);
			}
		}else{
			let new_top = Answer::new(bid, self.qid, base.len(), tqfs[self.aformula.0].conj.len());
			self.curr_answer_stack.push(new_top);
		}

		let mut curr_answer = self.curr_answer_stack.pop().unwrap();

		let mut i = 0;
		let start = self.answers.len();
		while i < limit{
			i = i + 1;
			match &curr_answer.state{
				MatchingState::Success | MatchingState::NextA | MatchingState::Zero => {
					curr_answer.next_a(psterms, &tqfs[self.aformula.0], base);
					continue;
				},
				MatchingState::NextB | MatchingState::Fail => {
					curr_answer.next_b();
					continue;
				},
				MatchingState::Ready => {
					match curr_answer.last().unwrap(){
						LogItem::Matching{batom_i, qatom_i, ..} => {
							let bterm = &base[*batom_i];
							if attributes.check(KeyObject::BaseIndex(*batom_i), AttributeName("deleted".to_string()), AttributeValue("true".to_string())){
								
								curr_answer.state = MatchingState::Fail;
								continue;
							}
							let btid = bterm.term;
							let qtid = tqfs[self.aformula.0].conj[*qatom_i];	

							if matching(btid, qtid, context, &mut curr_answer, psterms, base, attributes, bid){
								curr_answer.state = MatchingState::Success;
								continue;
							}else{
								curr_answer.state = MatchingState::Fail;
								continue;								
							}
						},
						LogItem::Interpretation{qatom_i} => {
														
							let qtid = tqfs[self.aformula.0].conj[*qatom_i];
							
							let mut env = PEnv{
								psterms: psterms,
								base: base,
								answer: &curr_answer,
								attributes: attributes,
								bid: bid,
							};

							let b = processing(qtid, context, Some(&curr_answer), &mut env).unwrap();
							if psterms.check_value(&b){
								curr_answer.state = MatchingState::Success;
							}else{
								curr_answer.state = MatchingState::Fail;
							}
							continue;
						},
					}
				},
				MatchingState::Rollback => {
					if curr_answer.len() > 0{
						curr_answer.state = MatchingState::NextB;
						continue;
					}else{
						curr_answer.state = MatchingState::NextK;
						continue;
					}
				},
				MatchingState::NextK => {
					curr_answer.next_k();
				},
				MatchingState::Exhausted => {
					if curr_answer.shift_bounds(base.len()){
						curr_answer.state = MatchingState::Zero;
					}else{
						break;
					}
				},
				MatchingState::Answer => {

					if curr_answer.conj_len == 0{
						curr_answer.state = MatchingState::Empty;	
					}else{
						curr_answer.state = MatchingState::NextB;
					}
					

					let na = curr_answer.clone();
					if let Some(_) = self.answers.iter().find(|x| *x == &na){
						continue;
					}				
					let aid = self.answers.len();	
					self.answers.push(na);

					let mut answer1 = self.answers.last().unwrap().clone();
					match si.selector{
						SelectorStrategy::First(f) => {
							if f(&answer1, &psterms){
								answer1.level = Some(level); 
								self.used_answers.push(answer1.clone());
								self.curr_answer_stack.push(curr_answer);
								return Some(AnswerId(self.qid.0, self.answers.len()-1))
							}else{
								continue;
							}
						},
						SelectorStrategy::Best(..) => {
							continue;
						}
					}
				},
				MatchingState::Empty => {
					break;
				}
			}

		}
		self.curr_answer_stack.push(curr_answer);

		let finish = self.answers.len();
		if let SelectorStrategy::Best(f) = si.selector{
			if self.answers.len() == 0{
				return None;
			}
			let res_answer = f(&self.answers, 0, &psterms);
			return res_answer;
		}

		None //		 
	}	
}












//
