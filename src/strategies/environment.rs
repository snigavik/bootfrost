
use crate::term::*;
use crate::base::*;
use crate::answer::*;
use crate::strategies::attributes::*;

pub struct PEnv<'a>{
	pub psterms: &'a mut PSTerms,
	pub base: &'a mut Base,
	pub answer: &'a Answer,
	pub attributes: &'a mut Attributes,
}