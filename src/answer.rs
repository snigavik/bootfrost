// use std::ops::Index;
use std::collections::HashMap;

use crate::misc::*;
//use crate::term::*;
use std::fmt;


#[derive(Clone)]
pub enum MatchingState{
	Ready,
	Fail,
	Success,
	Rollback,
	Zero,
	NextA, // ??
	NextB,
	NextK,
	Exhausted,
	Answer,
	Empty,
}

impl fmt::Debug for MatchingState{
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
    	match self{
    		MatchingState::Ready => write!(fmt,"Ready"),
    		MatchingState::Fail => write!(fmt,"Fail"),
    		MatchingState::Success => write!(fmt,"Success"),
    		MatchingState::Rollback => write!(fmt,"Rollback"),
    		MatchingState::Zero => write!(fmt,"Zero"),
    		MatchingState::NextA => write!(fmt,"NextA"),
    		MatchingState::NextB => write!(fmt,"NextB"),
    		MatchingState::NextK => write!(fmt,"NextK"),
    		MatchingState::Exhausted => write!(fmt,"Exhausted"),
    		MatchingState::Answer => write!(fmt,"Answer"),
    		MatchingState::Empty => write!(fmt,"Empty"),
    	}
    }
}


#[derive(Clone, Debug)]
pub enum LogItem{
	Matching{
		qatom_i: usize, 
		batom_i: usize, 
		avars:Vec<TermId>, 
	},
	Interpretation{
		qatom_i: usize, 
	},
}

// impl fmt::Debug for LogItem{
//     fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
//     	match self{
//     		LogItem::Matching{qatom_i, batom_i, avars} =>{
//     			write!(fmt,"qi: {}, bi: {}", qatom_i, batom_i)
//     		},
//     		LogItem::Interpretation{qatom_i} => {
//     			write!(fmt,"qi: {}", qatom_i)		
//     		} 
//     	}
//     }
// }



#[derive(Clone)]
pub struct Answer{
	pub amap: HashMap<TermId, TermId>,
	pub log: Vec<LogItem>,
	pub bid: BlockId, 
	pub qid: QuestionId,
	lower: usize,
	middle: usize,
	upper: usize,
	k: usize,
	pub conj_len: usize,
	pub state: MatchingState,	
}


impl fmt::Debug for Answer{
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
    	write!(fmt,"{}, {}, {}, {}, {}", self.lower, self.middle, self.upper, self.k, self.conj_len)
    }
}


impl Answer{


	pub fn new(bid: BlockId, qid: QuestionId, b_len: usize, q_len: usize) -> Self{
		Self{
			amap: HashMap::new(),
			log: vec![],
			bid: bid,
			qid: qid,
			lower: 0,
			middle: 0,
			upper: b_len-1,
			k: 0,
			conj_len: q_len,
			state: MatchingState::Zero,
		}
	}

	pub fn len(&self) -> usize{
		self.log.len()
	}

	pub fn last(&self) -> Option<&LogItem>{
		self.log.last()
	}
	pub fn last_mut(&mut self) -> Option<&mut LogItem>{
		self.log.last_mut()
	}	

	pub fn get(&self, tid:&TermId) -> Option<&TermId>{
		self.amap.get(tid)
	}

	pub fn push_satom(&mut self, qatom_i: usize, b:usize){
		self.log.push(LogItem::Matching{qatom_i: qatom_i, batom_i:b, avars: vec![]}); //FIX
	}

	pub fn push_iatom(&mut self, qatom_i: usize){
		self.log.push(LogItem::Interpretation{qatom_i: qatom_i});
	}

	pub fn push(&mut self, qtid: TermId, btid:TermId){
		self.amap.insert(qtid,btid);
		if let Some(last) = self.log.last_mut(){
			match last{
				LogItem::Matching{avars, ..} => {
					avars.push(qtid);
				},
				_ => panic!(""),
			}
		}else{
			panic!("");
		}

	}

	pub fn back_top(&mut self) -> bool{
		if let Some(last) = self.log.last_mut(){
			match last{
				LogItem::Matching{avars, ..} => {
					avars.iter().for_each(|v| {self.amap.remove(&v);});
					true
				},
				_ => true
			}			
		}else{
			false
		}
	}

	pub fn pop(&mut self) -> bool{
		if let Some(last) = self.log.pop(){
			match last{
				LogItem::Matching{avars, ..} => {
					avars.iter().for_each(|v| {self.amap.remove(&v);});
					true
				},
				_ => true
			}
		}else{
			false
		}
	}

	pub fn get_b(&self, qatom_i:usize) -> usize{
		if qatom_i < self.k{
			self.lower
		}else if qatom_i == self.k{
			self.middle
		}else{
			self.lower
		}
	}

	pub fn shift_bounds(&mut self, blen:usize) -> bool{
		if self.upper < blen-1{
			self.middle = self.upper;
			self.upper = blen-1;
			true
		}else{
			false
		}
	}

	pub fn next_k(&mut self){
		if self.k < self.conj_len - 1{
			self.k = self.k  + 1;
			self.state = MatchingState::Zero;
		}else{
			self.state = MatchingState::Exhausted;
		}
	}

	pub fn next_b(&mut self){
		match self.log.last_mut().unwrap(){
			LogItem::Matching{ref mut batom_i, qatom_i, ..} => {
				if *qatom_i < self.k{
					if *batom_i < self.middle{
						*batom_i = *batom_i + 1;
						self.back_top();
						self.state = MatchingState::Ready;
					}else{
						self.pop();
						self.state = MatchingState::Rollback;
					}
				}else if *qatom_i == self.k{
					if *batom_i < self.upper{
						*batom_i = *batom_i + 1;
						self.back_top();
						self.state = MatchingState::Ready;
					}else{
						self.pop();
						self.state = MatchingState::Rollback;
					}	
				}else if *qatom_i > self.k{
					if *batom_i < self.upper{
						*batom_i = *batom_i + 1;
						self.back_top();
						self.state = MatchingState::Ready;
					}else{
						self.pop();
						self.state = MatchingState::Rollback;
					}
				}
			},
			LogItem::Interpretation{..} => {
				self.pop();
				self.state = MatchingState::Rollback;
			},
		}
	}		
}



















//