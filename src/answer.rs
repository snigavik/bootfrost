// use std::ops::Index;
use std::collections::HashMap;

use crate::misc::*;
//use crate::term::*;

struct LogTuple{
	qatom: usize,
	batom: usize,
	avars: Vec<TermId>,
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
			last.avars.push(qtid);
		}else{
			panic!("");
		}

	}
}