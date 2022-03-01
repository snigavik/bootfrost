

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TermId(pub usize);

pub struct TqfId(pub usize);
pub struct ConjunctIndex(pub usize);
pub struct QuestionId(pub usize);
pub struct AnswerId(pub usize, pub usize);
pub struct NodeId(pub usize);


pub enum DisplayMode{
	Plain,
	Full,
}