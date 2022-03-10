

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TermId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TqfId(pub usize);
pub struct ConjunctIndex(pub usize);
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub usize);
pub struct AnswerId(pub usize, pub usize);

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BlockId(pub usize);


pub enum DisplayMode{
	Plain,
	Full,
}

