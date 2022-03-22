use crate::question::*;
use crate::term::*;
use crate::misc::*;

use std::collections::HashMap;

pub struct PlainTerm{
	pub symbol: String,
	pub args: Vec<PlainTerm>
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
	commands: Vec<PlainTerm>,
	next: Vec<PlainFormula>,
}

impl PlainFormula{
	pub fn new(q: String, vars: Vec<PlainTerm>, conj: Vec<PlainTerm>, commands:Vec<PlainTerm>, next: Vec<PlainFormula>) -> PlainFormula{
		PlainFormula{quantifier:q, vars: vars, conjunct: conj, commands: commands, next: next}
	}
}



fn plain_to_term(pt: PlainTerm, psterms: &mut PSTerms, vstack: &mut Vec<HashMap<String,TermId>>, smap: &mut HashMap<String, TermId>, fmap: &mut HashMap<String, SymbolId>) -> TermId{
	if let Some(m) = vstack.iter().rev().find(|&&vs| vs.contains_key(&pt.symbol)){
		*m.get(&pt.symbol).unwrap()
	}else{
		if pt.args.is_empty(){
			if let Some(tid) = smap.get(&pt.symbol){
				*tid
			}else{
				psterms.add_plain_const(pt.symbol, smap)
			}
		}else{
			psterms.add_plain_functor(pt, fmap)
		}
	}
}

pub fn plain_to_tqf(pf: PlainFormula, psterms: &mut PSTerms, vstack: &mut Vec<HashMap<String,TermId>>, smap: &mut HashMap<String, TermId>, fmap: &mut HashMap<String, SymbolId>, tqfs:&mut Vec<Tqf>) -> TqfId{
	
	let q = if pf.quantifier == "!".to_string(){
		Quantifier::Forall
	}else if pf.quantifier == "?".to_string(){
		Quantifier::Exists
	}else{
        panic!("Bad quantifier");
    };


    let mut vars = vec![];
    let mut vmap: HashMap<String, TermId> = HashMap::new();	
    for v in pf.vars{
    	let tid = psterms.add_plain_var(v.symbol, q);
    	vmap.insert(v.symbol, tid);
    	vars.push(tid);
    }
    vstack.push(vmap);


    let mut conj = vec![];
    for a in pf.conjunct{
    	conj.push(plain_to_term(a, psterms, vstack, smap, fmap));
    }


    let mut commands = vec![];
    for c in pf.commands{
    	commands.push(plain_to_term(c, psterms, vstack, smap, fmap));
    }


    let mut next = vec![];
    for n in pf.next{
    	next.push(plain_to_tqf(n, psterms, vstack, smap, fmap, tqfs));
    }

    tqfs.push(Tqf{quantifier:q, vars: vars, conj: conj, commands: commands, next: next});

    TqfId(tqfs.len() - 1)
}






//