// use std::ops::Index;
use std::collections::HashMap;

use crate::misc::*;
//use crate::term::*;


pub struct Context{
	map: HashMap<TermId, TermId>
}

impl Context{
	pub fn get(&self, tid:&TermId) -> Option<&TermId>{
		self.map.get(tid)
	}
}








//