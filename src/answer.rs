// use std::ops::Index;
use std::collections::HashMap;

use crate::misc::*;
//use crate::term::*;

// struct LogTuple{
// 	qatom: usize,
// 	batom: usize,
// 	avars: Vec<TermId>,
// }

enum LogTuple{
	Matching{qatom_i: usize, batom_i: usize, avars:Vec<TermId>},
	Interpretation{qatom_i: usize},
}

pub struct Answer{
	amap: HashMap<TermId, TermId>,
	log: Vec<LogTuple>,
	bid: BlockId, 
}

impl Answer{
	pub fn get(&self, tid:&TermId) -> Option<&TermId>{
		self.amap.get(tid)
	}

	pub fn push(&mut self, qtid: TermId, btid:TermId){
		self.amap.insert(qtid,btid);
		if let Some(last) = self.log.last_mut(){
			match last{
				LogTuple::Matching{avars, ..} => {
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
				LogTuple::Matching{avars, ..} => {
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




//