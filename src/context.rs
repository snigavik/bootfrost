use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::misc::*;
use crate::term::*;
use crate::answer::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context{
	pub map: HashMap<TermId, TermId>,
}

impl Context{
	pub fn new_empty() -> Context{
		Context{map: HashMap::new()}
	}


	pub fn new(prev_context: &Context, answer:&Answer) -> Self{
		let mut new_map = prev_context.map.clone();
		new_map.extend(answer.amap.clone().into_iter());

		Context{map: new_map}
	}	

	pub fn get(&self, tid:&TermId) -> Option<&TermId>{
		self.map.get(tid)
	}

	pub fn push_evars(&mut self, e_list: &Vec<TermId>, psterms: &mut PSTerms, bid:BlockId){
		for e in e_list.iter(){
			let e2 = psterms.new_e(*e, bid);
			self.map.insert(*e,e2);
		}
	}

	pub fn pop_evars(&mut self, e_list: &Vec<TermId>){
		for e in e_list.iter(){
			self.map.remove(e);
		}
	}
}








//