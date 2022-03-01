
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
	nid: NodeId,
	enable: bool,
}

struct Solver{
	psterms: PSTerms,
	base: Vec<BTerm>,
	base_index: BTreeMap<TermId, ConjunctIndex>,
	tqfs: Vec<Tqf>,
	questions: Vec<Question>,
	pstack: Vec<FBlock>,
	nid: NodeId,
}

impl Solver{


	fn matching(&self, btid:TermId, qtid:TermId, context: &Context, curr_answer: &mut Answer) -> bool{
		if btid == qtid{
			true
		}else{
			let bterm = &self.psterms[&btid];
			let qterm = &self.psterms[&qtid];
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
				Term::Functor(q_sid, q_args) => {
					match bterm{
						Term::Functor(b_sid, b_args) => {
							//
							true
						},
						_ => true
					}
				}
				_ => {
					panic!("");
				}
			}
		}
	}
}