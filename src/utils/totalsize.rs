
use std::collections::HashMap;

pub trait TotalSize {
    fn total_size(&self) -> usize;
}

impl<T: TotalSize> TotalSize for Vec<T>{
	fn total_size(&self) -> usize{
		self.iter().fold(0, |sum, x| sum + x.total_size())	
	}
}

impl<K: TotalSize, V: TotalSize> TotalSize for HashMap<K,V>{
	fn total_size(&self) -> usize{
		self.iter().fold(0, |sum, (k,v)| sum + k.total_size() + v.total_size())
	}
}