

pub struct PlainTerm{
	symbol: String,
	args: Vec<PlainTerm>
}

impl PlainTerm{
	pub fn new(symbol: String, args:Vec<PlainTerm>) -> PlainTerm{
		PlainTerm{symbol: symbol, args: args}
	}
}

pub struct PlainFormula{
	quantifier: String,
	vars: Vec<PlainTerm>,
	conjunct: Vec<PlainTerm>,
	next: Vec<PlainFormula>,
}

impl PlainFormula{
	pub fn new(q: String, vars: Vec<PlainTerm>, conj: Vec<PlainTerm>, next: Vec<PlainFormula>) -> PlainFormula{
		PlainFormula{quantifier:q, vars: vars, conjunct: conj, next: next}
	}
}

