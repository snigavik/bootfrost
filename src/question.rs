use crate::misc::*;

#[derive(Clone)]
pub enum Quantifier{
    Forall,
    Exists,
}

pub struct Tqf{
	vars: Vec<TermId>,
	conj: Vec<TermId>,
	commands: Vec<TermId>,
	next: Vec<TqfId>,
}


pub struct Question{
	aformula: TqfId,
	nid: NodeId,
}

