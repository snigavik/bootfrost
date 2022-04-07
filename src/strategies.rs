use crate::misc::*;
use crate::term::*;
use crate::question::*;
use crate::context::*;
use crate::answer::*;


pub struct StrategyItem{
	pub qid: QuestionId,
	pub selector: SelectorStrategy,
	pub sf: StartFrom,
	pub limit: usize,
}

pub enum SelectorStrategy{
	First(fn(&Answer, &PSTerms) -> bool),
	Best,
}

pub enum StartFrom{
	Last,
	Scratch,
}


pub fn plain_shift_strategy(questions: &Vec<Question>, step: usize) -> Vec<StrategyItem>{
	let mut vq:Vec<StrategyItem> = 
	questions
		.iter()
		.enumerate()
		.map(|(i,q)| 
			StrategyItem{
				qid: QuestionId(i),
				selector: SelectorStrategy::First(|x,y| true),
				sf: StartFrom::Last,
				limit:1000}).collect();
	vq.rotate_left(step % questions.len());	
	vq	
}


pub fn general_strategy(questions: &Vec<Question>, tqfs: &Vec<Tqf>, curr_level: usize) -> Vec<StrategyItem>{
	let mut state = questions
		.iter()
		.enumerate()
		.map(|(i,q)| (QuestionId(i), q.gs_rate(tqfs, curr_level, questions.len())))
		.collect::<Vec<(QuestionId, f64)>>();

	let disp1 = state.iter().map(|(q,x)| x.to_string()).collect::<Vec<String>>().join(", ");
	println!("{}", disp1);

	state.sort_by(|a,b| (a.1).partial_cmp(&b.1).unwrap());
		
	let vq: Vec<StrategyItem> = state
		.iter()
		.map(|(qid, r)|
			StrategyItem{
				qid: *qid,
				selector: SelectorStrategy::First(|x,y| true),
				sf: StartFrom::Last,
				limit:1000,
			}).collect();

	let disp2 = vq.iter().map(|x| (x.qid.0).to_string()).collect::<Vec<String>>().join(", ");
	println!("{}", disp2);
	vq
}



