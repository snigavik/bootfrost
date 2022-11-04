use serde::{Deserialize, Serialize};
// use crate::utils::totalsize::TotalSize;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct SymbolId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct TermId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct TqfId(pub usize);

pub struct ConjunctIndex(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct QuestionId(pub usize);

#[derive(Debug, Deserialize, Serialize)]
pub struct AnswerId(pub usize, pub usize);

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct BlockId(pub usize);

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Quantifier{
    Forall,
    Exists,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum DisplayMode{
	Plain,
	PlainSid,
	PlainSidTid
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Position{
	Classic,
	Infix,
}

#[derive(Debug, Deserialize, Serialize)]
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