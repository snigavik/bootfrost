use std::ops::Index;
use std::collections::HashMap;

use crate::misc::*;
use crate::term::*;

use crate::strategies::attributes::*;


pub struct Base{
	pub base: Vec<BTerm>,
	index: HashMap<TermId, usize>,
}

impl Base{
	pub fn new() -> Base{
		Base{base: vec![], index: HashMap::new()}
	}

	pub fn len(&self) -> usize{
		self.base.len()
	}

	pub fn is_empty(&self) -> bool{
		self.base.is_empty()
	}

	pub fn get_mut(&mut self, i:usize) -> Option<&mut BTerm>{
		self.base.get_mut(i)
	}

	pub fn push(&mut self, tid:TermId, bid: BlockId){
		self.index.insert(tid, self.base.len());
		self.base.push(BTerm{term: tid, bid: bid, deleted: false})
	}

	pub fn push_and_check(&mut self, tid:TermId, bid:BlockId, attributes: &Attributes) -> bool{
		if let Some(i) = self.index.get(&tid){
			let deleted = attributes.check(KeyObject::BaseIndex(*i), AttributeName("deleted".to_string()), AttributeValue("true".to_string()));
			// if self.base[*i].deleted{
			if deleted{
				self.push(tid, bid);
				true
			}else{
				false
			}
		}else{
			self.push(tid, bid);
			true
		}
	}

	pub fn remove(&mut self, bid:BlockId){
		while let Some(last) = self.base.last(){
			if last.bid == bid{
				if let Some(bt) = self.base.pop(){
					self.index.remove(&bt.term);
				}else{
					panic!("");
				}
			}else{
				break;
			}
		}		
	}

	pub fn deleted(&self, i:usize) -> bool{
		self.base[i].deleted
	}

	pub fn contains_key(&self, tid: &TermId) -> bool{
		self.index.contains_key(tid)
	}


}

impl Index<usize> for Base{
	type Output = BTerm;

	fn index (&self, i:usize) -> &Self::Output{
		&self.base[i]
	}
}