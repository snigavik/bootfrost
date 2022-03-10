
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

	fn eval_term(&mut self, tid:TermId) -> TermId{
		let t = &self.psterms.get_term(&tid);
		match t{
			Term::IFunctor(sid, args) => {
				let f = self.psterms.get_symbol(&sid).f;
				f(args, &mut self.psterms)
			},
			_ => tid
		}
	}


	fn matching(&mut self, btid:TermId, qtid:TermId, context: &Context, curr_answer: &mut Answer) -> bool{
		if btid == qtid{
			true
		}else{
			let bterm = self.psterms.get_term(&btid);
			let qterm = self.psterms.get_term(&qtid);
			match qterm{
				Term::AVariable(..) => {
					if let Some(new_qtid) = context.get(&qtid){
						self.matching(btid, *new_qtid, context, curr_answer)
					}else if let Some(new_qtid) = curr_answer.get(&qtid){
						self.matching(btid, *new_qtid, context, curr_answer)
					}else{
						curr_answer.push(qtid, btid);
						true
					}
				},
				Term::EVariable(..) => {
					if let Some(new_qtid) = context.get(&qtid){
						self.matching(btid, *new_qtid, context, curr_answer)
					}else{
						false
					}
				},
				Term::SFunctor(q_sid, q_args) => {
					match bterm{
						Term::SFunctor(b_sid,b_args) if q_sid == b_sid => {
							q_args.iter().zip(b_args.iter()).all(|pair| self.matching(*pair.1, *pair.0, context, curr_answer))
						},
						_ => false,
					}
				},
				Term::IFunctor(q_sid, q_args) => {
					let p = self.psterms.len();
					let new_qtid = self.eval_term(qtid);
					let m = self.matching(btid, new_qtid, context, curr_answer);
					self.psterms.back_to(p);
					m
				},
				_ => {
					panic!("");
				}
			}
		}
	}

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

	}

	fn nwxt_bounds(&mut self, qid: QuestionId){

	}

	fn proc(&mut self, qid:QuestionId){
		//let mut question = self.questions.get_mut(qid.0).unwrap();
		
		while true{
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
							let mut context = &mut self.pstack[self.questions[qid.0].fstack_i].context;
							let mut curr_answer = &mut self.question_mut(qid).answer_state.curr_answer;
							if self.matching(btid, qtid, context, &mut curr_answer){
								self.question_mut(qid).answer_state.state = MatchingState::Success;
								continue;
							}else{
								self.question_mut(qid).answer_state.state = MatchingState::Fail;
								continue;								
							}
						},
						LogItem::Interpretation{qatom_i} => {
							let qtid = self.tqf(self.questions[qid.0].aformula).conj[*qatom_i];
							self.eval_term(qtid); // CHECK false
							continue;
						},
					}
				},
				MatchingState::Rollback => {

				},
				MatchingState::NextK => {
					//self.question_mut(qid).answer_state.state = MatchingState::Zero;
				},
				MatchingState::Exhausted => {

				}
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
