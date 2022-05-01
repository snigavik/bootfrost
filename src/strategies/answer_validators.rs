use crate::misc::*;
use crate::term::*;
use crate::answer::*;

use std::io::stdin;


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


pub fn best_manual(answers: &Vec<Answer>, psterms: &PSTerms) -> bool{

	true
}




