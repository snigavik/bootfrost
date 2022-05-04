use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use crate::misc::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct AttributeName(pub String);


#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct AttributeValue(pub String);

#[derive(Hash, Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum KeyObject{
	BaseIndex(usize),
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Attributes{
	map: HashMap<KeyObject, HashMap<AttributeName, Vec<(AttributeValue, BlockId)>>>,
	index: HashMap<BlockId, Vec<(KeyObject, AttributeName)>>,
}

impl Attributes{
	pub fn new() -> Attributes{
		Attributes{
			map: HashMap::new(),
			index: HashMap::new(),
		}
	}

	pub fn set_attribute(&mut self, obj: KeyObject, attr: AttributeName, v: AttributeValue, bid: BlockId){
		if let Some(v) = self.index.get_mut(&bid){
			v.push((obj.clone(), attr.clone()));
		}else{
			self.index.insert(bid, vec![(obj.clone(), attr.clone())]);
		}		

		if let Some(obj) = self.map.get_mut(&obj){
			if let Some(a) = obj.get_mut(&attr){
				a.push((v, bid));
			}else{
				obj.insert(attr, vec![(v, bid)]);
			}
		}else{
			let mut hm = HashMap::new();
			hm.insert(attr, vec![(v, bid)]);
			self.map.insert(obj, hm);
		}
	}

	pub fn remove_bid(&mut self, bid: BlockId){
		if let Some(vector) = self.index.get(&bid){
			for (keyobj, attrname ) in vector{
				if let Some(value) = self.map.get_mut(keyobj){
					if let Some(attr) = value.get_mut(attrname){
						attr.retain(|(_,bid1)| *bid1 != bid);
					}
				}
			}
		}
	}

	// check the attribute
	// return false if the attribute doesn't exist
	pub fn check(&self, obj: KeyObject, attr: AttributeName, v: AttributeValue) -> bool{
		if let Some(obj) = self.map.get(&obj){
			if let Some(a) = obj.get(&attr){
				if let Some((value, _bid)) = a.last(){
					if *value == v{
						true
					}else{
						false
					}
				}else{
					false
				}
			}else{
				false
			}
		}else{
			false
		}
	}
}