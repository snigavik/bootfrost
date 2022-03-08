use crate::misc::*;
use crate::answer::*;

#[derive(Clone)]
pub enum Quantifier{
    Forall,
    Exists,
}

pub struct Tqf{
	quantifier: Quantifier,
	vars: Vec<TermId>,
	conj: Vec<TermId>,
	commands: Vec<TermId>,
	next: Vec<TqfId>,
}



pub struct Question{
	aformula: TqfId,
	bid: BlockId,
	answer_state: AnswerState,
	// curr_answer: Answer,
	// bounds: Bounds,
}

