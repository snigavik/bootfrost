use crate::misc::*;
use crate::term::*;
use crate::base::*;

pub struct PEnv<'a>{
	pub psterms: &'a mut PSTerms,
	pub base: &'a mut Base,
}