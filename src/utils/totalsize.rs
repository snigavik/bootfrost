
use std::collections::HashMap;
use core::mem::size_of_val;

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


//Need to generalize by macros
//Basic types

impl TotalSize for usize{
	fn total_size(&self) -> usize{
		size_of_val(self)
	}
}

impl TotalSize for String{
	fn total_size(&self) -> usize{
		size_of_val(self)
	}
}

impl TotalSize for i64{
	fn total_size(&self) -> usize{
		size_of_val(self)
	}
}

impl TotalSize for bool{
	fn total_size(&self) -> usize{
		size_of_val(self)
	}
}





