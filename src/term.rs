
use std::fmt;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Index;
use std::hash::{Hash, Hasher};


use crate::misc::*;
use crate::context::*;
use crate::plain::PlainTerm;


pub enum FunctorType{
	None,
	SFunctor,
	IFunctor(fn(&Vec<TermId>, &mut PSTerms) -> TermId),
}

pub struct Symbol{
	pub uid: usize,
	pub name: String,
	pub f: Option<fn(&Vec<TermId>, &mut PSTerms) -> TermId>
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Term{
	AVariable(SymbolId),
	EVariable(SymbolId, BlockId),
	SConstant(SymbolId),
	Bool(bool),
	Integer(i64),
	String(String),
	SFunctor(SymbolId, Vec<TermId>),
	IFunctor(SymbolId, Vec<TermId>,),
}

pub struct BTerm{
	pub term: TermId,
	pub bid: BlockId,
	pub deleted: bool,
}



pub struct PSTerms{
	symbols: Vec<Symbol>,
	terms: Vec<Term>,
	index: HashMap<Term,TermId>,	
}

impl Index<&TermId> for PSTerms{
	type Output = Term;

	fn index (&self, tid: &TermId) -> &Self::Output{
		&self.terms[tid.0]
	}
}

impl Index<&SymbolId> for PSTerms{
	type Output = Symbol;

	fn index (&self, sid: &SymbolId) -> &Self::Output{
		&self.symbols[sid.0]
	}
}


impl PSTerms{

	pub fn add_plain_functor(&mut self, pt:PlainTerm, fmap: &mut HashMap<String, SymbolId>) -> TermId{
		if let Some(_sid) = fmap.get(&pt.symbol){
			if let Some(sid) = self.symbols.get(_sid.0){
				if let Some(..) = sid.f{
					let term = Term::IFunctor(*_sid, pt.args.into_iter().map(|a| self.add_plain_functor(a, fmap)).collect());
					if let Some(tid) = self.index.get(&term){
						return *tid;
					}else{
		            	let tid = TermId(self.terms.len());
		            	self.terms.push(term.clone());
		            	self.index.insert(term, tid);
		            	return tid;						
					}
				}else{
					let term = Term::SFunctor(*_sid, pt.args.into_iter().map(|a| self.add_plain_functor(a, fmap)).collect());
					if let Some(tid) = self.index.get(&term){
						return *tid;
					}else{
		            	let tid = TermId(self.terms.len());
		            	self.terms.push(term.clone());
		            	self.index.insert(term, tid);
		            	return tid;						
					}
				}
			}else{
				panic!("");
			}
		}else{
			let sid = SymbolId(self.symbols.len());
			fmap.insert(pt.symbol, sid);
			let term = Term::SFunctor(sid, pt.args.into_iter().map(|a| self.add_plain_functor(a, fmap)).collect());
			let tid = TermId(self.terms.len());
			self.terms.push(term.clone());
			self.index.insert(term,tid);
			return tid;
		}
	}

	pub fn add_plain_const(&mut self, s:String, smap: &mut HashMap<String, TermId>) -> TermId{
        if let Ok(n) = &s.parse::<i64>(){
            let term = Term::Integer(*n);
            if let Some(tid) = self.index.get(&term){
            	return *tid;
            }else{
            	let tid = TermId(self.terms.len());
            	self.terms.push(term);
            	return tid;
            }
        }

        if s == "true"{
            return TermId(1);
        }

        if s == "false"{
            return TermId(0);
        }

        if s.starts_with("\"") && s.ends_with("\""){
			let mut s1 = s.clone();
			s1.remove(0);
			s1.remove(s1.len()-1);

		    let term = Term::String(s1);
            if let Some(tid) = self.index.get(&term){
            	return *tid;
            }else{
            	let tid = TermId(self.terms.len());
            	self.terms.push(term);
            	return tid;
            }
        }

		if let Some(tid) = smap.get(&s){
			return *tid;
		}else{
			let sid = self.symbols.len();
			self.symbols.push(Symbol{uid: sid, name: s.clone(), f: None});
			let term = Term::SConstant(SymbolId(sid));
			let tid = TermId(self.terms.len());
			self.terms.push(term.clone());
			self.index.insert(term, tid);
			smap.insert(s.clone(), tid);
			return tid;	
		}		
	}

	pub fn add_plain_var(&mut self, v:String, q:Quantifier) -> TermId{
		let sid = self.symbols.len();
		self.symbols.push(Symbol{uid: sid, name: v, f: None});
		let term = match q{
			Quantifier::Forall => {
				Term::AVariable(SymbolId(sid))
			},
			Quantifier::Exists => {
				Term::EVariable(SymbolId(sid),BlockId(0))
			},
		};
		let tid = TermId(self.terms.len());
		self.terms.push(term.clone());
		self.index.insert(term, tid);
		tid	
	}



	pub fn is_false(&self, tid:&TermId) -> bool{
		match self.terms[tid.0]{
			Term::Bool(b) if b == false => {
				true
			},
			_ => false
		}
	}

	pub fn check_value(&self, tid:&TermId) -> bool{
		match self.terms[tid.0]{
			Term::Bool(b) => b,
			_ => {
				panic!("");
			},
		}
	}

	pub fn new_e(&mut self, e: TermId, bid: BlockId) -> TermId{
		let sid = match self.get_term(&e){
			Term::EVariable(sid, ..) =>{
				sid
			},
			_ => {
				panic!("");
			}
		};
		let new_tid = TermId(self.terms.len());
		self.terms.push(Term::EVariable(sid, bid));
		new_tid
	} 

	pub fn get_tid(&mut self, term:Term) -> ProcessingResult{
		if let Some(tid) = self.index.get(&term){
			ProcessingResult::Existing(*tid)
		}else{
			let new_tid = TermId(self.terms.len());
			self.terms.push(term.clone());
			self.index.insert(term.clone(), new_tid);
			ProcessingResult::New(new_tid)
		}
	}

	pub fn get_term(&self, tid:&TermId) -> Term{
		//self.terms.get(tid.0)
		self.terms[tid.0].clone()
	}

	pub fn get_symbol(&self, sid:&SymbolId) -> &Symbol{
		//self.symbols.get(sid.0)
		&self.symbols[sid.0]
	}


	pub fn len(&self) -> usize{
		self.terms.len()
	}

	pub fn back_to(&mut self, car: usize){
		while self.terms.len() > car{
			if let Some(t) = self.terms.pop(){
				self.index.remove(&t);
			}
		}
	}
}


//https://doc.rust-lang.org/rust-by-example/hello/print.html


// pub struct SymbolDisplay<'a>(pub &'a Symbol, pub &'a DisplayMode);
// impl fmt::Display for AVariableSymbolDisplay<'_>{
//     fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
//     	match self.1{
//     		DisplayMode::Plain => write!(fmt, "{}", self.0.name),
//     		DisplayMode::Full => write!(fmt, "{}.{}", self.0.name, self.0.uid),
//     	}
//     }
// }


// struct TermDisplay<'a>(&'a TermId, &'a PSTerms, Option<&'a Context>, &'a DisplayMode);
// struct TermsDisplay<'a>(&'a Vec<TermId>, &'a PSTerms, Option<&'a Context>, &'a DisplayMode);


// impl fmt::Display for TermDisplay<'_>{
//     fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
// 		match &self.1[self.0]{
// 			Term::AVariable(s, vd, vi) => { 
// 				if let Some(context) = self.2{
// 					write!(fmt,"{}",AVariableSymbolDisplay(&*s,self.3))
// 				}else{
// 					write!(fmt,"{}",AVariableSymbolDisplay(&*s,self.3))
// 				}
// 			},
// 			_ => {
// 				write!(fmt,"ok")
// 			}
// 		}
// 	}
// }



// impl fmt::Display for TermsDisplay<'_>{
//     fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
//     	write!(fmt,"{}",self.0.iter().map(|x| TermDisplay(&x,self.1,self.2).to_string()).collect::<Vec<String>>().join(",")) 
//     }
// }



