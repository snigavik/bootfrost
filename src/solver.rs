
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

	// fn eval_term(&mut self, tid:TermId) -> TermId{
	// 	let t = &self.psterms.get_term(&tid);
	// 	match t{
	// 		Term::IFunctor(sid, args) => {
	// 			let f = self.psterms.get_symbol(&sid).f;
	// 			f(args, &mut self.psterms)
	// 		},
	// 		_ => tid
	// 	}
	// }


	// fn matching(&mut self, btid:TermId, qtid:TermId, context: &Context, curr_answer: &mut Answer) -> bool{
	// 	if btid == qtid{
	// 		true
	// 	}else{
	// 		let bterm = self.psterms.get_term(&btid);
	// 		let qterm = self.psterms.get_term(&qtid);
	// 		match qterm{
	// 			Term::AVariable(..) => {
	// 				if let Some(new_qtid) = context.get(&qtid){
	// 					self.matching(btid, *new_qtid, context, curr_answer)
	// 				}else if let Some(new_qtid) = curr_answer.get(&qtid){
	// 					self.matching(btid, *new_qtid, context, curr_answer)
	// 				}else{
	// 					curr_answer.push(qtid, btid);
	// 					true
	// 				}
	// 			},
	// 			Term::EVariable(..) => {
	// 				if let Some(new_qtid) = context.get(&qtid){
	// 					self.matching(btid, *new_qtid, context, curr_answer)
	// 				}else{
	// 					false
	// 				}
	// 			},
	// 			Term::SFunctor(q_sid, q_args) => {
	// 				match bterm{
	// 					Term::SFunctor(b_sid,b_args) if q_sid == b_sid => {
	// 						q_args.iter().zip(b_args.iter()).all(|pair| self.matching(*pair.1, *pair.0, context, curr_answer))
	// 					},
	// 					_ => false,
	// 				}
	// 			},
	// 			Term::IFunctor(q_sid, q_args) => {
	// 				let p = self.psterms.len();
	// 				let new_qtid = self.eval_term(qtid);
	// 				let m = self.matching(btid, new_qtid, context, curr_answer);
	// 				self.psterms.back_to(p);
	// 				m
	// 			},
	// 			_ => {
	// 				panic!("");
	// 			}
	// 		}
	// 	}
	// }

	fn next_a(&mut self, qid: QuestionId){
		let mut question = self.questions.get_mut(qid.0).unwrap();
		let state_len = question.answer_state.curr_answer.len();
		let conj_len = question.answer_state.conj_len;
		if state_len < conj_len{
			let x = &self.tqfs[question.aformula.0].conj[state_len];
			let q_term = self.psterms.get_term(x);
			question.answer_state.state = MatchingState::Ready;
			match q_term{
				Term::SFunctor(..) => {
					question.answer_state.curr_answer.push_satom(state_len);
				},
				Term::IFunctor(..) => {
					question.answer_state.curr_answer.push_iatom(state_len);
				},
				_ => {
					panic!("");
				}
			}
		}else{
			question.answer_state.state = MatchingState::NextB;
		}	
	}

	fn next_b(&mut self, qid: QuestionId){
		let mut question = self.questions.get_mut(qid.0).unwrap();
		question.answer_state.next_b();
	}

	fn next_k(&mut self, qid: QuestionId){
		let mut question = self.questions.get_mut(qid.0).unwrap();
		question.answer_state.next_k();
	}

	fn next_bounds(&mut self, qid: QuestionId) -> bool{
		let mut question = self.questions.get_mut(qid.0).unwrap();
		let blen = self.base.len();
		question.answer_state.shift_bounds(blen)
	}

	//TODO: Check boundary conditions everywhere
	fn proc(&mut self, qid:QuestionId, limit:usize){
		//let mut question = self.questions.get_mut(qid.0).unwrap();
		let mut i = 0;
		while i < limit{
			i = i + 1;
			match self.questions[qid.0].answer_state.state{
				MatchingState::Success | MatchingState::NextA | MatchingState::Zero => {
					self.next_a(qid);
					continue;
				},
				MatchingState::NextB | MatchingState::Fail => {
					self.next_b(qid);
					continue;
				},
				MatchingState::Ready => {
					match self.questions[qid.0].answer_state.curr_answer.last().unwrap(){
						LogItem::Matching{batom_i, qatom_i, ..} => {
							let bterm = &self.base[*batom_i];
							if bterm.deleted{
								self.question_mut(qid).answer_state.state = MatchingState::Fail;
								continue;
							}
							let btid = bterm.term;
							let qtid = self.tqf(self.questions[qid.0].aformula).conj[*qatom_i];
							let context = &self.pstack[self.questions[qid.0].fstack_i].context;
							// let curr_answer = &mut self.question_mut(qid).answer_state.curr_answer;
							let mut curr_answer = &mut self.questions.get_mut(qid.0).unwrap().answer_state.curr_answer;
							if matching(&mut self.psterms, btid, qtid, context, curr_answer){
								self.question_mut(qid).answer_state.state = MatchingState::Success;
								continue;
							}else{
								self.question_mut(qid).answer_state.state = MatchingState::Fail;
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
					if self.questions[qid.0].answer_state.curr_answer.len() > 0{
						self.question_mut(qid).answer_state.state = MatchingState::NextB;
						continue;
					}else{
						self.question_mut(qid).answer_state.state = MatchingState::NextK;
						continue;
					}
				},
				MatchingState::NextK => {
					self.next_k(qid);
				},
				MatchingState::Exhausted => {
					if self.next_bounds(qid){
						self.question_mut(qid).answer_state.state = MatchingState::Zero;
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
