use serde::{Deserialize, Serialize};

use crate::misc::*;
use crate::term::*;
use crate::question::*;

use crate::answer::*;

use crate::strategies::environment::*;
use crate::strategies::answer_validators::*;

use std::io::stdin;


pub struct StrategyItem{
	pub qid: QuestionId,
	pub selector: SelectorStrategy,
	pub sf: StartFrom,
	pub limit: usize,
}

#[derive(Clone)]
pub enum SelectorStrategy{
	First(fn(&Answer, &PSTerms) -> bool),
	Best(fn(&Vec<Answer>, usize, &PSTerms) -> Option<AnswerId>),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StartFrom{
	Last,
	Scratch,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Strategy{
	PlainShift,
	General,
	ManualFirst,
	ManualBest,
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


fn read_questions_list(qlen: usize) -> Vec<usize>{
	loop{
		let mut try_again = false;
		let mut res:Vec<usize> = vec![];

		println!("Type list of questions separated by commas (example: 0,2,1)");
		println!("Each question must be a positive number in the range [0..{}]",qlen);
	    let mut input_string = String::new();
	    stdin().read_line(&mut input_string)
	    	.ok()
	        .expect("Failed to read line");	
	    if input_string.trim() == "q"{
	    	panic!("");
	    }
	    let q_list:Vec<&str> = input_string.trim().split(",").collect();

	    for (i,x) in q_list.iter().enumerate(){
	    	if i == q_list.len()-1 && x.trim() == ".."{
	    		let r_full:Vec<usize> = (0..qlen+1).collect();
	    		for r in r_full{
	    			if !res.contains(&r){
	    				res.push(r);
	    			}
	    		}
	    		return res;
	    	}
	    	let q = match x.trim().parse::<usize>(){
	    		Ok(n) => {
	    			if n > qlen{
	    				try_again = true;
	    				println!("Question #{} is out of possible range [0..{}]. Try again.", n, qlen);
	    				break;
	    			}else{
	    				n
	    			}
	    		},
	    		Err(_) => {
	    			try_again = true;
	    			println!("Erorr. Invalid item: {}. Question must be a positive number. Try again.", x);
	    			break;
	    		},
	    	};
	    	res.push(q);
	    }
	    if try_again{
	    	continue;
	    }else{
	    	return res;
	    }
	}
}


pub fn manual_strategy(questions: &Vec<Question>, selector: SelectorStrategy) -> Vec<StrategyItem>{
    let mut vq: Vec<StrategyItem> = vec![];

 	let q_list = read_questions_list(questions.len()-1);

    for q in q_list{

    	let si = StrategyItem{
    		qid: QuestionId(q),
    		selector: selector.clone(), //SelectorStrategy::First(first_manual),
    		sf: StartFrom::Last,
    		limit:1000,
    	};
    	vq.push(si);
    }

    vq
}


