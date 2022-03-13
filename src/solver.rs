
use std::collections::BTreeMap;

use crate::misc::*;
use crate::term::*;
use crate::question::*;
use crate::context::*;
use crate::answer::*;

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
		let state_len = question.answer.len();
		let conj_len = question.answer.conj_len;
		if state_len < conj_len{
			let x = &self.tqfs[question.aformula.0].conj[state_len];
			let q_term = self.psterms.get_term(x);
			question.answer.state = MatchingState::Ready;
			match q_term{
				Term::SFunctor(..) => {

					question.answer.push_satom(state_len, question.answer.get_b(state_len));
					true
				},
				Term::IFunctor(..) => {
					question.answer.push_iatom(state_len);
					true
				},
				_ => {
					panic!("");
				}
			}
		}else{
			question.answer.state = MatchingState::NextB;
			false
		}	
	}

	fn next_b(&mut self, qid: QuestionId){
		let mut question = self.questions.get_mut(qid.0).unwrap();
		question.answer.next_b();
	}

	fn next_k(&mut self, qid: QuestionId){
		let mut question = self.questions.get_mut(qid.0).unwrap();
		question.answer.next_k();
	}

	fn next_bounds(&mut self, qid: QuestionId) -> bool{
		let mut question = self.questions.get_mut(qid.0).unwrap();
		let blen = self.base.len();
		question.answer.shift_bounds(blen)
	}

	//TODO: Check boundary conditions everywhere
	fn proc(&mut self, qid:QuestionId, limit:usize){
		//let mut question = self.questions.get_mut(qid.0).unwrap();
		let mut i = 0;
		while i < limit{
			i = i + 1;
			match self.questions[qid.0].answer.state{
				MatchingState::Success | MatchingState::NextA | MatchingState::Zero => {
					if !self.next_a(qid){
						
					}
					continue;
				},
				MatchingState::NextB | MatchingState::Fail => {
					self.next_b(qid);
					continue;
				},
				MatchingState::Ready => {
					match self.questions[qid.0].answer.last().unwrap(){
						LogItem::Matching{batom_i, qatom_i, ..} => {
							let bterm = &self.base[*batom_i];
							if bterm.deleted{
								self.question_mut(qid).answer.state = MatchingState::Fail;
								continue;
							}
							let btid = bterm.term;
							let qtid = self.tqf(self.questions[qid.0].aformula).conj[*qatom_i];
							let context = &self.pstack[self.questions[qid.0].fstack_i].context;
							let mut curr_answer = &mut self.questions.get_mut(qid.0).unwrap().answer;
							if matching(&mut self.psterms, btid, qtid, context, curr_answer){
								self.question_mut(qid).answer.state = MatchingState::Success;
								continue;
							}else{
								self.question_mut(qid).answer.state = MatchingState::Fail;
								continue;								
							}
						},
						LogItem::Interpretation{qatom_i} => {
							let qtid = self.tqf(self.questions[qid.0].aformula).conj[*qatom_i];
							eval_term(&mut self.psterms, qtid); // CHECK false
							continue;
						},
					}
				},
				MatchingState::Rollback => {
					if self.questions[qid.0].answer.len() > 0{
						self.question_mut(qid).answer.state = MatchingState::NextB;
						continue;
					}else{
						self.question_mut(qid).answer.state = MatchingState::NextK;
						continue;
					}
				},
				MatchingState::NextK => {
					self.next_k(qid);
				},
				MatchingState::Exhausted => {
					if self.next_bounds(qid){
						self.question_mut(qid).answer.state = MatchingState::Zero;
					}else{
						break;
					}
				}
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
