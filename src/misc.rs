

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SymbolId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TermId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TqfId(pub usize);
pub struct ConjunctIndex(pub usize);
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub usize);
pub struct AnswerId(pub usize, pub usize);

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct BlockId(pub usize);

#[derive(Copy, Clone)]
pub enum Quantifier{
    Forall,
    Exists,
}


pub enum DisplayMode{
	Plain,
	Full,
}

pub enum ProcessingResult{
	New(TermId),
	Existing(TermId),
	Error,
}

impl ProcessingResult{
	pub fn unwrap(&self) -> TermId{
		match self{
			ProcessingResult::New(tid) | ProcessingResult::Existing(tid) => *tid,
			_ => {
				panic!("");
			}
		}
	}
}