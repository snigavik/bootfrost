// use std::ops::Index;
use std::collections::HashMap;

use crate::misc::*;
//use crate::term::*;


pub enum MatchingState{
	Ready,
	Fail,
	Success,
	RollBack,
}

enum LogItem{
	Matching{
		qatom_i: usize, 
		batom_i: usize, 
		avars:Vec<TermId>, 
	},
	Interpretation{
		qatom_i: usize, 
	},
}

pub struct Answer{
	amap: HashMap<TermId, TermId>,
	log: Vec<LogItem>,
	bid: BlockId, 
}



impl Answer{
	pub fn len(&self) -> usize{
		self.log.len()
	}

	pub fn get(&self, tid:&TermId) -> Option<&TermId>{
		self.amap.get(tid)
	}

	pub fn push_satom(&mut self, qatom_i: usize){
		self.log.push(LogItem::Matching{qatom_i: qatom_i, batom_i:0, avars: vec![]});
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

	pub fn back(&mut self) -> bool{
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
}

pub struct AnswerState{
	lower: usize,
	middle: usize,
	upper: usize,
	k: usize,
	pub conj_len: usize,
	pub state: MatchingState,
	pub curr_answer: Answer,
}

















//