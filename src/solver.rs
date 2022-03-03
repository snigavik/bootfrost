
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

	fn eval_term(&mut self, tid:TermId) -> TermId{
		let t = &self.psterms.get_term(&tid);
		match t{
			Term::IFunctor(sid, args, f) => {
				f(args, &mut self.psterms)
			},
			_ => tid
		}
	}


	fn matching(&mut self, btid:TermId, qtid:TermId, context: &Context, curr_answer: &mut Answer) -> bool{
		if btid == qtid{
			true
		}else{
			let bterm = &self.psterms.get_term(&btid);
			let qterm = &self.psterms.get_term(&qtid);
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
				Term::IFunctor(q_sid, q_args, _) => {
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
}