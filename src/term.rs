
use crate::strategies::environment::PEnv;
use std::fmt;
use std::collections::HashMap;
use std::ops::Index;
use std::hash::Hash;


use crate::misc::*;
use crate::context::*;
use crate::plain::*;
use crate::strategies::environment::*;


pub enum FunctorType{
	None,
	SFunctor,
	IFunctor(fn(&Vec<TermId>, &mut PSTerms) -> TermId),
}

pub struct Symbol{
	pub uid: usize,
	pub name: String,
	pub f: Option<fn(&Vec<TermId>, &mut PEnv) -> TermId>,
	pub position: Position,
}

impl fmt::Debug for Symbol{
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
    	write!(fmt,"symbol: {}, {}",self.uid, self.name)
    }
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
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

#[derive(Debug)]
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

	pub fn add_ifunction(&mut self, name:String, f: Option<fn(&Vec<TermId>, &mut PEnv) -> TermId>, position: Position,) -> SymbolId{
		let sid = self.symbols.len();
		self.symbols.push(Symbol{uid:sid, name:name, f:f, position: position});

		SymbolId(sid)
	}



	pub fn print_symbols(&self){
		for s in &self.symbols{
			println!("{}.{}",s.name, s.uid);
		}
	}

	pub fn new() -> PSTerms{
		PSTerms{symbols: vec![], terms: vec![Term::Bool(false), Term::Bool(true)], index: HashMap::from([(Term::Bool(false),TermId(0)), (Term::Bool(true),TermId(1))])}
	}

	pub fn add_plain_functor(&mut self, pt:PlainTerm, vstack: &mut Vec<HashMap<String,TermId>>, smap: &mut HashMap<String, TermId>, fmap: &mut HashMap<String, SymbolId>) -> TermId{
		if let Some(_sid) = fmap.get(&pt.symbol){
			if let Some(sid) = self.symbols.get(_sid.0){
				if let Some(..) = sid.f{
					let term = Term::IFunctor(*_sid, pt.args.into_iter().map(|a| plain_to_term(a, self, vstack, smap, fmap)).collect());
					if let Some(tid) = self.index.get(&term){
						return *tid;
					}else{
		            	let tid = TermId(self.terms.len());
		            	self.terms.push(term.clone());
		            	self.index.insert(term, tid);
		            	return tid;						
					}
				}else{
					let term = Term::SFunctor(*_sid, pt.args.into_iter().map(|a| plain_to_term(a, self, vstack, smap, fmap)).collect());
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
			self.symbols.push(Symbol{uid: sid.0, name: pt.symbol.clone(), f: None, position: pt.position});
			fmap.insert(pt.symbol, sid);
			let term = Term::SFunctor(sid, pt.args.into_iter().map(|a| plain_to_term(a, self, vstack, smap, fmap)).collect());
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
            	self.terms.push(term.clone());
            	self.index.insert(term,tid);
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
            	self.terms.push(term.clone());
            	self.index.insert(term,tid);
            	return tid;
            }
        }

		if let Some(tid) = smap.get(&s){
			return *tid;
		}else{
			let sid = self.symbols.len();
			self.symbols.push(Symbol{uid: sid, name: s.clone(), f: None, position: Position::Classic});
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
		self.symbols.push(Symbol{uid: sid, name: v, f: None, position: Position::Classic});
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
		self.terms[tid.0].clone()
	}

	pub fn get_symbol(&self, sid:&SymbolId) -> &Symbol{
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

pub struct SidDisplay<'a>{
	pub sid: SymbolId, 
	pub psterms: &'a PSTerms,
	pub dm: DisplayMode
}

impl fmt::Display for SidDisplay<'_>{
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
    	let symbol = self.psterms.get_symbol(&self.sid);
    	match self.dm{
    		DisplayMode::Plain => write!(fmt, "{}", symbol.name),
    		DisplayMode::PlainSid | DisplayMode::PlainSidTid => write!(fmt, "{}.{}", symbol.name, symbol.uid),
    	}
    }
}

pub struct TidDisplay<'a>{
	pub tid: TermId,
	pub psterms: &'a PSTerms,
	pub context: Option<&'a Context>,
	pub dm: DisplayMode
}


impl fmt::Display for TidDisplay<'_>{
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
    	let term = self.psterms.get_term(&self.tid);
		match term{
			Term::AVariable(sid) => { 
				if let Some(context) = self.context{
					if let Some(new_tid) = context.get(&self.tid){
						write!(fmt,"{}",
							TidDisplay{
								tid: *new_tid,
								psterms: self.psterms,
								context: Some(context),
								dm: self.dm,
							}
						)
					}else{
						write!(fmt,"{}",SidDisplay{sid: sid, psterms: self.psterms, dm: self.dm})	
					}
				}else{
					write!(fmt,"{}",SidDisplay{sid: sid, psterms: self.psterms, dm: self.dm})
				}
			},
			Term::EVariable(sid, bid) => { 
				if let Some(context) = self.context{
					if let Some(new_tid) = context.get(&self.tid){
						write!(fmt,"{}",
							TidDisplay{
								tid: *new_tid,
								psterms: self.psterms,
								context: Some(context),
								dm: self.dm,
							}
						)
					}else{
						match self.dm{
							DisplayMode::PlainSid | DisplayMode::PlainSidTid => write!(fmt,"{}.{}",SidDisplay{sid: sid, psterms: self.psterms, dm: self.dm}, bid.0),
							_ => write!(fmt,"{}",SidDisplay{sid: sid, psterms: self.psterms, dm: self.dm}),
						}
					}
				}else{
					match self.dm{
						DisplayMode::PlainSid | DisplayMode::PlainSidTid => write!(fmt,"{}.{}",SidDisplay{sid: sid, psterms: self.psterms, dm: self.dm}, bid.0),
						_ => write!(fmt,"{}",SidDisplay{sid: sid, psterms: self.psterms, dm: self.dm}),
					}					
				}
			},
			Term::SConstant(sid) => {
				write!(fmt,"{}", SidDisplay{sid:sid, psterms: self.psterms, dm: DisplayMode::Plain})
			},
			Term::Bool(b) => {
				write!(fmt,"{}", b)
			},
			Term::Integer(i) => {
				write!(fmt, "{}", i)
			},
			Term::String(s) => {
				write!(fmt, "\"{}\"", s)
			},
			Term::SFunctor(sid, args) | Term::IFunctor(sid, args) => {
				let sd = &SidDisplay{sid: sid, psterms: self.psterms, dm:DisplayMode::Plain}.to_string();
				match self.psterms.get_symbol(&sid).position{
					Position::Classic => {
						write!(fmt,"{}({})", 
							sd,
							TidsDisplay{
								tids: &args,
								psterms: self.psterms,
								context: self.context,
								dm: self.dm,
								d: ", "
							}.to_string()
						)
					},
					Position::Infix => {
						let mut sd1 = " ".to_string();
						sd1.push_str(sd);
						sd1.push_str(" ");
						write!(fmt,"{}",
							TidsDisplay{
								tids: &args,
								psterms: self.psterms,
								context: self.context,
								dm: self.dm,
								d: &sd1,
							}.to_string()
						)						
					}
				}

			},
		}
	}
}


pub struct TidsDisplay<'a>{
	pub tids: &'a Vec<TermId>,
	pub psterms: &'a PSTerms,
	pub context: Option<&'a Context>,
	pub dm: DisplayMode,
	pub d: &'a str,
}

impl fmt::Display for TidsDisplay<'_>{
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
		write!(fmt, "{}",
			self.tids.iter().map(|a| 
				TidDisplay{
					tid: *a,
					psterms: self.psterms,
					context: self.context,
					dm: self.dm,							
				}.to_string()).collect::<Vec<String>>().join(self.d)  
		)  	
    }
}


