use std::collections::HashMap;
use crate::misc::*;

pub struct AttributeName{
	name: String,
}

pub struct AttributeValue{
	bid: BlockId,
	value: String,
}

pub enum KeyObject{
	BaseIndex(usize),
}


pub struct Attributes{
	map: HashMap<KeyObject, HashMap<AttributeName, Vec<AttributeValue>>>,
}

impl Attributes{
	pub fn new() -> Attributes{
		Attributes{
			map: HashMap::new()
		}
	}
}