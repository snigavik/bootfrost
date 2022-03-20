// use std::ops::Index;
use std::collections::HashMap;

use crate::misc::*;
use crate::term::*;
use crate::answer::*;


pub struct Context{
	map: HashMap<TermId, TermId>
}

impl Context{
	pub fn new(prev_context: &Context, answer:&Answer, e_list: &Vec<TermId>, psterms: &mut PSTerms) -> Self{
		Context{map: HashMap::new()}
	}

	pub fn get(&self, tid:&TermId) -> Option<&TermId>{
		self.map.get(tid)
	}
}








//