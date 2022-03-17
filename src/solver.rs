
use std::collections::BTreeMap;

use crate::misc::*;
use crate::term::*;
use crate::question::*;
use crate::context::*;
use crate::answer::*;

struct StrategyItem{
	qid: QuestionId,
	selector: SelectorStrategy,
	sf: StartFrom,
	limit: usize,
}

enum SelectorStrategy{
	First(fn(&Answer, &PSTerms) -> bool),
	Best,
}

enum StartFrom{
	Last,
	Scratch,
}


struct FBlock{
	qid: QuestionId,
	aid: AnswerId,
	eid: TqfId,
	context: Context,
	bid: BlockId,
	enable: bool,
}

struct Solver{
	psterms: PSTerms,
	base: Vec<BTerm>,
	base_index: BTreeMap<TermId, ConjunctIndex>,
	tqfs: Vec<Tqf>,
	questions: Vec<Question>,
	pstack: Vec<FBlock>,
	bid: BlockId,
}

impl Solver{

	fn question_mut(&mut self, i:QuestionId) -> &mut Question{
		if let Some(q) = self.questions.get_mut(i.0){
			q
		}else{
			panic!("");
		}
	}

	fn tqf(&self, i: TqfId) -> &Tqf{
		if let Some(tqf) = self.tqfs.get(i.0){
			tqf
		}else{
			panic!("")
		}
	}

	fn next_a(&mut self, qid: QuestionId) -> bool{
		let mut question = self.questions.get_mut(qid.0).unwrap();
		let state_len = question.curr_answer_stack.last_mut().unwrap().len();
		let conj_len = question.curr_answer_stack.last_mut().unwrap().conj_len;
		if state_len < conj_len{
			let x = &self.tqfs[question.aformula.0].conj[state_len];
			let q_term = self.psterms.get_term(x);
			question.curr_answer_stack.last_mut().unwrap().state = MatchingState::Ready;
			match q_term{
				Term::SFunctor(..) => {
					if self.base.len() == 0{
						question.curr_answer_stack.last_mut().unwrap().state = MatchingState::Exhausted;
						false
					}else{
						let b = question.curr_answer_stack.last().unwrap().get_b(state_len);
						question.curr_answer_stack.last_mut().unwrap().push_satom(state_len, b);
						true
					}
				},
				Term::IFunctor(..) => {
					question.curr_answer_stack.last_mut().unwrap().push_iatom(state_len);
					true
				},
				_ => {
					panic!("");
				}
			}
		}else{
			question.curr_answer_stack.last_mut().unwrap().state = MatchingState::Answer;
			false
		}	
	}

	fn next_b(&mut self, qid: QuestionId){
		let mut question = self.questions.get_mut(qid.0).unwrap();
		question.curr_answer_stack.last_mut().unwrap().next_b();
	}

	fn next_k(&mut self, qid: QuestionId){
		let mut question = self.questions.get_mut(qid.0).unwrap();
		question.curr_answer_stack.last_mut().unwrap().next_k();
	}

	fn next_bounds(&mut self, qid: QuestionId) -> bool{
		let mut question = self.questions.get_mut(qid.0).unwrap();
		let blen = self.base.len();
		question.curr_answer_stack.last_mut().unwrap().shift_bounds(blen)
	}

	//TODO: Check boundary conditions everywhere
	fn proc(&mut self, si: &StrategyItem, bid: BlockId) -> Option<Answer>{
		let qid = si.qid;
		let limit = si.limit;
		if let Some(top) = self.questions[qid.0].curr_answer_stack.last(){
			if top.bid != bid{
				let mut new_top = top.clone();
				new_top.bid = bid;
				self.questions[qid.0].curr_answer_stack.push(new_top);
			}
		}else{
			let new_top = Answer::new(bid, self.base.len(), self.tqf(self.questions[qid.0].aformula).conj.len());
			self.questions[qid.0].curr_answer_stack.push(new_top);
		}


		let mut i = 0;
		while i < limit{
			i = i + 1;
			match self.questions[qid.0].curr_answer_stack.last_mut().unwrap().state{
				MatchingState::Success | MatchingState::NextA | MatchingState::Zero => {
					self.next_a(qid);
					continue;
				},
				MatchingState::NextB | MatchingState::Fail => {
					self.next_b(qid);
					continue;
				},
				MatchingState::Ready => {
					match self.questions[qid.0].curr_answer_stack.last().unwrap().last().unwrap(){
						LogItem::Matching{batom_i, qatom_i, ..} => {
							let bterm = &self.base[*batom_i];
							if bterm.deleted{
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Fail;
								continue;
							}
							let btid = bterm.term;
							let qtid = self.tqf(self.questions[qid.0].aformula).conj[*qatom_i];
							let context = &self.pstack[self.questions[qid.0].fstack_i].context;
							let mut curr_answer = &mut self.questions.get_mut(qid.0).unwrap().curr_answer_stack.last_mut().unwrap();
							if matching(&mut self.psterms, btid, qtid, context, curr_answer){
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Success;
								continue;
							}else{
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Fail;
								continue;								
							}
						},
						LogItem::Interpretation{qatom_i} => {
							let qtid = self.tqf(self.questions[qid.0].aformula).conj[*qatom_i];
							let b = eval_term(&mut self.psterms, qtid);
							if self.psterms.check_value(&b){
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Success;
							}else{
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Fail;
							}
							continue;
						},
					}
				},
				MatchingState::Rollback => {
					if self.questions[qid.0].curr_answer_stack.last_mut().unwrap().len() > 0{
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::NextB;
						continue;
					}else{
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::NextK;
						continue;
					}
				},
				MatchingState::NextK => {
					self.next_k(qid);
				},
				MatchingState::Exhausted => {
					if self.next_bounds(qid){
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Zero;
					}else{
						break;
					}
				},
				MatchingState::Answer => {
					let nq =self.questions[qid.0].curr_answer_stack.last_mut().unwrap().clone();
					self.questions.get_mut(qid.0).unwrap().answers.push(nq);

					if self.questions[qid.0].curr_answer_stack.last_mut().unwrap().conj_len == 0{
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Empty;	
					}else{
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::NextB;
					}
				},
				MatchingState::Empty => {
					break;
				}
			}

		}
		None		 
	}

	fn strategy(&self) -> Vec<StrategyItem>{
		vec![]
	}

	pub fn solver_loop(&mut self){
		let bid = BlockId(1000);
		let strategy = self.strategy();
		for si in strategy.iter(){
			if let Some(answer) = self.proc(si, bid){

				break;
			}
		}
	}
}



fn eval_term(psterms: &mut PSTerms,  tid:TermId) -> TermId{
	let t = &psterms.get_term(&tid);
	match t{
		Term::IFunctor(sid, args) => {
			let f = psterms.get_symbol(&sid).f;
			f(args, psterms)
		},
		_ => tid
	}
}

fn matching(psterms: &mut PSTerms, btid:TermId, qtid:TermId, context: &Context, curr_answer: &mut Answer) -> bool{
	if btid == qtid{
		true
	}else{
		let bterm = psterms.get_term(&btid);
		let qterm = psterms.get_term(&qtid);
		match qterm{
			Term::AVariable(..) => {
				if let Some(new_qtid) = context.get(&qtid){
					matching(psterms, btid, *new_qtid, context, curr_answer)
				}else if let Some(new_qtid) = curr_answer.get(&qtid){
					matching(psterms, btid, *new_qtid, context, curr_answer)
				}else{
					curr_answer.push(qtid, btid);
					true
				}
			},
			Term::EVariable(..) => {
				if let Some(new_qtid) = context.get(&qtid){
					matching(psterms, btid, *new_qtid, context, curr_answer)
				}else{
					false
				}
			},
			Term::SFunctor(q_sid, q_args) => {
				match bterm{
					Term::SFunctor(b_sid,b_args) if q_sid == b_sid => {
						q_args.iter().zip(b_args.iter()).all(|pair| matching(psterms, *pair.1, *pair.0, context, curr_answer))
					},
					_ => false,
				}
			},
			Term::IFunctor(q_sid, q_args) => {
				let p = psterms.len();
				let new_qtid = eval_term(psterms, qtid);
				let m = matching(psterms, btid, new_qtid, context, curr_answer);
				psterms.back_to(p);
				m
			},
			_ => {
				panic!("");
			}
		}
	}
}


// fn next_a(qid: QuestionId, s: &mut Solver) -> bool{
// 	let mut question = s.questions.get_mut(qid.0).unwrap();
// 	let state_len = question.answer_state.curr_answer.len();
// 	let conj_len = question.answer_state.conj_len;
// 	if state_len < conj_len{
// 		let x = &s.tqfs[question.aformula.0].conj[state_len];
// 		let q_term = s.psterms.get_term(x);
// 		question.answer_state.state = MatchingState::Ready;
// 		match q_term{
// 			Term::SFunctor(..) => {
// 				question.answer_state.curr_answer.push_satom(state_len);
// 				true
// 			},
// 			Term::IFunctor(..) => {
// 				question.answer_state.curr_answer.push_iatom(state_len);
// 				true
// 			},
// 			_ => {
// 				panic!("");
// 			}
// 		}
// 	}else{
// 		false
// 	}	
// }
