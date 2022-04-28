use crate::misc::*;
use crate::term::*;
use crate::question::*;

use crate::answer::*;

use crate::strategies::environment::*;

use std::io::stdin;

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

pub enum Strategy{
	PlainShift,
	General,
	Manual,
	User,
}


pub fn plain_shift_strategy(questions: &Vec<Question>, step: usize) -> Vec<StrategyItem>{
	let mut vq:Vec<StrategyItem> = 
	questions
		.iter()
		.enumerate()
		.map(|(i,_q)| 
			StrategyItem{
				qid: QuestionId(i),
				selector: SelectorStrategy::First(|_,_| true),
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

	let disp1 = state.iter().map(|(_q,x)| x.to_string()).collect::<Vec<String>>().join(", ");
	println!("{}", disp1);

	state.sort_by(|a,b| (a.1).partial_cmp(&b.1).unwrap());
		
	let vq: Vec<StrategyItem> = state
		.iter()
		.map(|(qid, _r)|
			StrategyItem{
				qid: *qid,
				selector: SelectorStrategy::First(|_,_| true),
				sf: StartFrom::Last,
				limit:1000,
			}).collect();

	let disp2 = vq.iter().map(|x| (x.qid.0).to_string()).collect::<Vec<String>>().join(", ");
	println!("{}", disp2);
	vq
}

pub fn manual_strategy(questions: &Vec<Question>) -> Vec<StrategyItem>{
	println!("Type questions from 0 to {}: ", questions.len()-1);
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");

    let mut vq: Vec<StrategyItem> = vec![];

    let q_list = input_string.trim().split(","); //.map(|x| x.parse::<usize>().unwrap());
    for x in q_list{
    	// println!("{}",x);
    	let q = x.parse::<usize>().unwrap();
    	let si = StrategyItem{
    		qid: QuestionId(q),
    		selector: SelectorStrategy::First(|answer, psterms|{
    			println!("Do you accept this answer: {}", AnswerDisplay{answer: answer, psterms: psterms, dm: DisplayMode::Plain});
    			let mut inp = String::new();
    			stdin().read_line(&mut inp)
    				.ok()
    				.expect("Failed to read line");
    			match inp.trim(){
    				"y" => true,
    				"n" => false,
    				_ => {panic!("");}
    			}
    		}),
    		sf: StartFrom::Last,
    		limit:1000,
    	};
    	vq.push(si);
    }

    vq
}


