
use std::fmt;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Index;
use std::hash::{Hash, Hasher};


use crate::misc::*;
use crate::context::*;


pub enum FunctorType{
	None,
	SFunctor,
	IFunctor(fn(&Vec<TermId>, &mut PSTerms) -> TermId),
}

pub struct Symbol{
	uid: usize,
	name: String,
	//f: FunctorType,
	pub f: fn(&Vec<TermId>, &mut PSTerms) -> TermId
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



