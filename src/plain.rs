use crate::question::*;
use crate::term::*;
use crate::misc::*;

use std::collections::HashMap;

pub struct PlainTerm{
	pub symbol: String,
	pub args: Vec<PlainTerm>,
	pub position: Position,
	pub complex: bool,
}

impl PlainTerm{
	pub fn new(symbol: String, args:Vec<PlainTerm>, complex: bool) -> PlainTerm{
		PlainTerm{symbol: symbol, args: args, position: Position::Classic, complex: complex}
	}
	pub fn new_infix(symbol: String, args:Vec<PlainTerm>) -> PlainTerm{
		PlainTerm{symbol: symbol, args: args, position: Position::Infix, complex: true}
	}

	pub fn print(&self){
		print!("{}", self.symbol);
		if self.args.len() > 0{
			print!("(");
			for a in &self.args{
				a.print();
				print!(",");
			}
			print!(")");
		}
	}
}
 
pub struct PlainFormula{
	pub quantifier: String,
	pub vars: Vec<PlainTerm>,
	pub conjunct: Vec<PlainTerm>,
	pub commands: Vec<PlainTerm>,
	pub next: Vec<PlainFormula>,
}

impl PlainFormula{
	pub fn new(q: String, vars: Vec<PlainTerm>, conj: Vec<PlainTerm>, commands:Vec<PlainTerm>, next: Vec<PlainFormula>) -> PlainFormula{
		PlainFormula{quantifier:q, vars: vars, conjunct: conj, commands: commands, next: next}
	}

	pub fn print(&self, tab:String){
		print!("{}", tab);
		print!("{}", self.quantifier);
		for v in &self.vars{
			v.print();
		}
		print!("  ");
		for a in &self.conjunct{
			a.print();
			print!(",");
		}

		if self.commands.len() > 0{
			print!("  $  ");
			for c in &self.commands{
				c.print();
				print!(",");
			}
		}

		println!("");
		if self.next.len() > 0 {
			for n in &self.next{
				let mut new_tab = tab.clone();
				new_tab.push_str("    ");
				n.print(new_tab);
			}
		}
 	}
}



pub fn plain_to_term(pt: PlainTerm, psterms: &mut PSTerms, vstack: &mut Vec<HashMap<String,TermId>>, smap: &mut HashMap<String, TermId>, fmap: &mut HashMap<String, SymbolId>) -> TermId{
	if let Some(m) = vstack.iter().rev().find(|&vs| vs.contains_key(&pt.symbol)){
		*m.get(&pt.symbol).unwrap()
	}else{
		// if pt.args.is_empty(){
		if !pt.complex{
			if let Some(tid) = smap.get(&pt.symbol){
				*tid
			}else{
				psterms.add_plain_const(pt.symbol, smap)
			}
		}else{
			psterms.add_plain_functor(pt, vstack, smap, fmap)
		}
	}
}

pub fn plain_to_tqf(
	pf: PlainFormula, 
	psterms: &mut PSTerms, 
	vstack: &mut Vec<HashMap<String,TermId>>, 
	smap: &mut HashMap<String, TermId>, 
	fmap: &mut HashMap<String, SymbolId>, 
	tqfs: &mut Vec<Tqf>) -> TqfId{
	
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
    	let tid = psterms.add_plain_var(v.symbol.clone(), q);
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