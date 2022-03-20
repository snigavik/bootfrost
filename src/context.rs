// use std::ops::Index;
use std::collections::HashMap;

use crate::misc::*;
use crate::term::*;
use crate::answer::*;

pub struct Context{
	pub map: HashMap<TermId, TermId>
}

impl Context{
	pub fn new(prev_context: &Context, answer:&Answer, e_list: &Vec<TermId>, psterms: &mut PSTerms, bid:BlockId) -> Self{
		let mut new_map = prev_context.map.clone();
		new_map.extend(answer.amap.clone().into_iter());
		for e in e_list.iter(){
			let e2 = psterms.new_e(*e, bid);
			new_map.insert(*e,e2);
		}

		Context{map: new_map}
	}

	pub fn get(&self, tid:&TermId) -> Option<&TermId>{
		self.map.get(tid)
	}
}








//