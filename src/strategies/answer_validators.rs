use crate::misc::*;
use crate::term::*;
use crate::answer::*;

use std::io::stdin;

use crate::strategies::{strategies::AnswerOption};

pub fn first_manual(answer: &Answer, psterms: &PSTerms) -> bool{
	loop{
		let mut try_again = false;
		println!("Do you accept this answer [y/n]: {}", AnswerDisplay{answer: answer, psterms: psterms, dm: DisplayMode::Plain});
		let mut inp = String::new();
		stdin().read_line(&mut inp)
			.ok()
			.expect("Failed to read line");
		match inp.trim(){
			"y" => {
				return true;
			},
			"n" => {
				return false;
			},
			"q" => {
				panic!("");
			},
			_ => {
				println!("Type y or n.");
				try_again = true;
			}
		}
		if try_again{
			continue;
		}
	}	
}


pub fn best_manual(answers: &Vec<Answer>, used_answers: &Vec<Answer>, start:usize, psterms: &PSTerms) -> AnswerOption{
	let new_len = answers.len() - start;
	println!("\nSelect your answer (type corresponding number)");
	for (i, x) in answers[start..].iter().enumerate(){
		let mut mark = "";
		if used_answers.contains(&answers[i]){
			mark = " ,used";
		}
		println!("({}{}) {}",i, mark, AnswerDisplay{answer: x, psterms: psterms, dm: DisplayMode::Plain});
	}

	loop{
		let mut try_again = false;
		let mut inp = String::new();
		stdin().read_line(&mut inp)
			.ok()
			.expect("Failed to read line");
		
		match inp.trim(){
			"n" => {
				return AnswerOption::Next;
			},
			"r" => {
				return AnswerOption::Restart;
			},			
			"q" => {
				panic!("");
			},
			_ => {

			}			
		}

		let na = match inp.trim().parse::<usize>(){
			Ok(n) => {
				if n >= new_len{
					try_again = true;
					println!("Answer #{} is out of possible range [0..{}]. Try again.", n, new_len-1);
					continue;
				}else{
					n
				}
			},
			Err(_) => {
				try_again = true;
				println!("Erorr. Invalid item: {}. Answer must be a positive number. Try again.", inp);
				continue;
			},			
		};
		if try_again{
			continue;
		}	
		return AnswerOption::Success(AnswerId(answers[start+na].qid.0, start+na));	
	}
}




