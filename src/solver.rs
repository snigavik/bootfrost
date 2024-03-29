use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::mem;

use crate::misc::*;
use crate::term::*;
use crate::question::*;
use crate::context::*;
use crate::answer::*;
use crate::plain::*;
use crate::base::*;
use crate::solverlog::*;

use crate::strategies::{environment::*, attributes::*, strategies::*, ifunctions::*, answer_validators::*};


#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum SolverResultType{
	Refuted,
	Exhausted,
	LimitReached,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SolverResult{
	pub t: SolverResultType,
	pub steps: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BranchBlock{
	pub aid: AnswerId,
	pub atqf: TqfId,
	pub eindex: usize,
	pub context: Context,
	pub bid: BlockId,
	pub psterms_car: usize,
	pub enabled: bool,
}


#[derive(Serialize)]
pub struct Solver{
	psterms: PSTerms,
	base: Base,
	tqfs: Vec<Tqf>,
	questions: Vec<Question>,
	bstack: Vec<BranchBlock>,
	curr_bid: BlockId,
	curr_step: usize,
	attributes: Attributes,
	strategy: Strategy,
	pub slog: SolverLog,
}

impl Solver{

	pub fn print_tqf(&self, tid: TqfId, tab:String, context: &Context){
		let tqf = &self.tqfs[tid.0];
		print!("{}", tab);
		match tqf.quantifier{
			Quantifier::Forall => {
				print!("!");
			},
			Quantifier::Exists => {
				print!("?");
			}
		}

		// vars
		print!("{}", TidsDisplay{
			tids: &tqf.vars,
			psterms: &self.psterms,
			context: Some(context),
			dm: DisplayMode::PlainSid,
			d: ",",
		});

		// conj
		print!(" ");
		print!("{}", TidsDisplay{
			tids: &tqf.conj,
			psterms: &self.psterms,
			context: Some(context),
			dm: DisplayMode::PlainSid,
			d: ", ",
		});

		// commands
		print!(" ");
		if !tqf.commands.is_empty(){
			print!("$ ");
			print!("{}", TidsDisplay{
				tids: &tqf.commands,
				psterms: &self.psterms,
				context: Some(context),
				dm: DisplayMode::PlainSid,
				d: ", ",
			});			
		}

		println!("");
		let mut new_tab = tab.clone();
		new_tab.push_str("    ");
		for n in &tqf.next{
			self.print_tqf(*n, new_tab.clone(), context);
		}		

	}


	pub fn print(&mut self){
		println!("\nCurrent formula:");
		print!("Base: ");
		let mut b_str = "".to_string();

		for (i,b) in self.base.base.iter().enumerate(){
			let deleted = self.attributes.check(KeyObject::BaseIndex(i), AttributeName("deleted".to_string()), AttributeValue("true".to_string()));
			if deleted{ print!("["); b_str = format!("{}[",b_str);}

			let td1 = TidDisplay{
				tid: b.term,
				psterms: &self.psterms,
				context: None,
				dm: DisplayMode::PlainSid,
			};

			print!("{}", &td1);
			b_str = format!("{}{}",b_str,&td1);

			if deleted{ print!("]");b_str = format!("{}]",b_str);}			
			if i < self.base.len() - 1{ print!(", "); b_str = format!("{}, ",b_str);}

			
		}
		if !self.slog.is_empty(){
			self.slog.set_base(b_str);
		}

		println!("\n\nQuestions:");
		for (i,q) in self.questions.iter().enumerate(){
			print!("({}) ", i);
			self.print_tqf(q.aformula, "".to_string(), &self.bstack[q.fstack_i].context);
		}
	}

	pub fn parse_file(path: &str, strategy: Strategy) -> Solver{
		let pf = crate::parser::parser::parse_file(path);
		Solver::from_pf(pf, strategy)
	}

	pub fn parse_string(s: &str, strategy: Strategy) -> Solver{
		let pf = crate::parser::parser::parse_string(s);
		Solver::from_pf(pf, strategy)
	}

	pub fn from_pf(pf: PlainFormula, strategy: Strategy) -> Solver{

		let mut vstack = vec![];
		let mut smap = HashMap::from([("false".to_string(),TermId(0)), ("true".to_string(),TermId(1))]);

		let mut tqfs = vec![];
		let (mut psterms, mut fmap) = crate::strategies::ifunctions::init();

		let tid = plain_to_tqf(pf, &mut psterms, &mut vstack, &mut smap, &mut fmap, &mut tqfs);

		let first_block: BranchBlock = BranchBlock{
			aid: AnswerId(1000000000, 1000000000),
			atqf: tid,
			eindex: 0,
			context: Context::new_empty(),
			bid: BlockId(1),
			psterms_car: psterms.len(),
			enabled: false,
		};


		let mut solver = Solver{
			psterms: psterms,
			base: Base::new(),
			tqfs: tqfs,
			questions: vec![],
			bstack: vec![first_block],
			curr_bid: BlockId(0),
			curr_step:0,
			attributes: Attributes::new(),
			strategy: strategy,
			slog: SolverLog::new()
		};

		solver.enable_block();
		solver
	}

	fn level(&self) -> usize{
		self.bstack.len() - 1
	}


	fn strategy(&self) -> Vec<StrategyItem>{		
		
		match self.strategy{
			Strategy::PlainShift => {
				plain_shift_strategy(&self.questions, self.curr_step)
			},
			Strategy::General => {
				let curr_level = self.bstack.len() - 1;
				general_strategy(&self.questions, &self.tqfs, curr_level)
			},
			Strategy::ManualFirst => {
				manual_strategy(&self.questions, SelectorStrategy::First(first_manual))
			},
			Strategy::ManualBest => {
				manual_strategy(&self.questions, SelectorStrategy::Best(best_manual))
			},			
			Strategy::User => {
				panic!("");
			},
		}
	}

	fn find_answer_global(&mut self) -> AnswerOption{
		let bid = self.bstack.last().unwrap().bid;
		let strategy = self.strategy();
		for si in strategy.iter(){
			let fstack_i = self.questions[si.qid.0].fstack_i;
			println!("Try question {}", si.qid.0);
			let res = self.questions[si.qid.0].find_answer_local(si, bid, &mut self.psterms, &self.tqfs, &mut self.base, self.bstack.len()-1, &self.bstack[fstack_i].context, &mut self.attributes);
			match res{
				AnswerOption::Success(aid) => {
					let answer = &self.questions[aid.0].answers[aid.1];
					println!("{}: {}",si.qid.0, AnswerDisplay{answer: &answer, psterms: &self.psterms, dm: DisplayMode::Plain});
					self.slog.set_qa(si.qid.0, format!("{}",AnswerDisplay{answer: &answer, psterms: &self.psterms, dm: DisplayMode::Plain}));
					return AnswerOption::Success(aid);
				},
				AnswerOption::Next => {
					continue;
				},
				AnswerOption::Restart => {
					return AnswerOption::Restart;
				},
				AnswerOption::Fail => {
					println!("No answers have been found (selected).");
				},			
			}
			// if let Some(aid) = self.questions[si.qid.0].find_answer_local(si, bid, &mut self.psterms, &self.tqfs, &mut self.base, self.bstack.len()-1, &self.bstack[fstack_i].context, &mut self.attributes){
			// 	let answer = &self.questions[aid.0].answers[aid.1];
			// 	println!("{}: {}",si.qid.0, AnswerDisplay{answer: &answer, psterms: &self.psterms, dm: DisplayMode::Plain});
			// 	return Some(aid);
			// }else{
			// 	println!("No answers have been found.");
			// }
		}
		
		match self.strategy{
			Strategy::ManualBest => AnswerOption:: Restart,
			_ => AnswerOption::Fail,
		}
	}


	fn transform(&mut self, aid:AnswerId){
		let answer = &self.questions[aid.0].answers[aid.1];
		let curr_context = &self.bstack[self.questions[aid.0].fstack_i].context;	
		let origin_bid = self.bstack[self.questions[aid.0].fstack_i].bid;	
		let a_tqf = &self.questions[aid.0].aformula;
		let e_tqfs = &self.tqfs[a_tqf.0].next;
		let atqf = self.questions[aid.0].aformula;

		if e_tqfs.len() == 0{
			self.remove_branch();
			self.enable_block_loop();
			return;
		}

		let commands = &self.tqfs[a_tqf.0].commands;

		self.curr_bid = BlockId(self.curr_bid.0 + 1);

		let mut env = PEnv{
			psterms: &mut self.psterms,
			base: &mut self.base,
			answer: &answer,
			attributes: &mut self.attributes,
			bid: self.curr_bid,
		};
		commands.iter().for_each(|c| {processing(*c, &curr_context, Some(&answer), &mut env);});

		

		let mut new_block: BranchBlock = BranchBlock{
			aid: aid,
			atqf: atqf,
			eindex: 0,
			context: Context::new(&curr_context, &answer),
			bid: self.curr_bid,
			psterms_car:self.psterms.len(),
			enabled: false,
		};

		self.bstack.push(new_block);
		self.enable_block();
	}

	fn disable_block(&mut self){
		if let Some(top) = self.bstack.last_mut(){
			if top.enabled{
				self.base.remove(top.bid);

				self.questions.retain(|q| q.bid != top.bid);

				self.questions.iter_mut().for_each(|q| q.remove_answers(top.bid));

				self.attributes.remove_bid(top.bid);

				let eid = &self.tqfs[top.atqf.0].next[top.eindex];
				let etqf = &self.tqfs[eid.0];
				let evars = &etqf.vars;

				top.context.pop_evars(evars);

				self.psterms.back_to(top.psterms_car);

				top.enabled = false;
			}else{
				panic!("");
			}
		}else{
			panic!("");
		}
	}

	fn enable_block_loop(&mut self){
		while self.bstack.len() > 0{
			if !self.enable_block(){
				self.remove_branch();
			}else{
				return;
			}	 
		}		
	}


	fn enable_block(&mut self) -> bool{
		let fstack_i = self.bstack.len() - 1;
		let level = self.bstack.len();
		if let Some(top) = self.bstack.last_mut(){
			top.enabled = true;
			top.psterms_car = self.psterms.len();
			let eid = &self.tqfs[top.atqf.0].next[top.eindex];
			let etqf = &self.tqfs[eid.0];
			let econj = &etqf.conj;
			let evars = &etqf.vars;

			top.context.push_evars(evars, &mut self.psterms, top.bid);

			let new_conj: Vec<TermId> = if level > 1{
				let mut env = PEnv{
					psterms: &mut self.psterms,
					base: &mut self.base,
					answer: &self.questions[top.aid.0].answers[top.aid.1],
					attributes: &mut self.attributes,
					bid: top.bid,
				};				

				econj
					.iter()
					.map(|a| 
						processing(*a, &top.context, None, &mut env).unwrap())
					.collect()
			}else{
				econj.clone()
			};

			let mut added_terms = vec![];
			let mut skipped_terms = vec![];
			for a in new_conj{
				if a == TermId(0){
					println!("False has been occurred");
					return false;
				}

				let r = self.base.push_and_check(a,top.bid, &self.attributes);
				if r{
					added_terms.push(a);
				}else{
					skipped_terms.push(a);
				}
			}	
			if level > 1{
				println!("Terms added to the base: {}", TidsDisplay{tids: &added_terms, psterms: &self.psterms, context:None, dm: DisplayMode::Plain, d:", "});
				let mut env = PEnv{
					psterms: &mut self.psterms,
					base: &mut self.base,
					answer: &self.questions[top.aid.0].answers[top.aid.1],
					attributes: &mut self.attributes,
					bid: top.bid,
				};				
				print_batoms(&vec![], &mut env);
				
				let mut vector = env.answer.get_batoms();
				vector.retain(|s| s.is_some());
				let a_u_str = vector.iter().map(|ve|
					format!("{}, ", TidDisplay{
						tid: env.base[ve.unwrap()].term,
						psterms: env.psterms,
						context: None,
						dm: DisplayMode::Plain,}
					)
				).collect::<Vec<String>>().join(", ");
				
				let a_a_str = format!("{}", TidsDisplay{tids: &added_terms, psterms: &self.psterms, context:None, dm: DisplayMode::Plain, d:", "});

				self.slog.set_atoms(a_a_str, a_u_str);

			}
			// add questions
			let a_tqfs = &etqf.next;
			let q_len = self.questions.len();
			let mut new_questions: Vec<Question> = 
				a_tqfs
					.iter()
					.enumerate()
					.map(|(i,af)| 
						Question{
							qid: QuestionId(q_len + i),
							bid: top.bid,
							aformula: *af,
							fstack_i: fstack_i,
							curr_answer_stack: vec![],
							answers: vec![],
							used_answers: vec![],
						}).collect();
			if new_questions.len() > 0{
				println!("\nNew questions: YES. ({})-({})", self.questions.len(), self.questions.len() + new_questions.len() - 1)
			}else{
				println!("\nNew questions: NO.")
			}
			self.questions.append(&mut new_questions);
			return true;		
		}else{
			panic!("");
		}
		true
	}

	pub fn next_block(&mut self) -> bool{
		if let Some(top) = self.bstack.last_mut(){
			let e_tqfs = &self.tqfs[top.atqf.0].next;
			let esize = e_tqfs.len();
			if top.eindex < esize - 1{
				top.eindex = top.eindex + 1;
				self.curr_bid = BlockId(self.curr_bid.0 + 1); 
				top.bid = self.curr_bid;
				true
			}else{
				false
			}
		}else{
			panic!("");
		}
	}

	pub fn remove_branch(&mut self){
		while let Some(..) = self.bstack.last(){
			self.disable_block();
			if !self.next_block(){
				self.bstack.pop();
			}else{
				break;
			}
		}
		println!("Branch has been removed");

	}



	pub fn solver_loop(&mut self, limit:usize) -> SolverResult{
		let mut i = 0;
		while i < limit{
			println!("=========================================================================");
			println!("================================ Step {} ================================", self.curr_step);
			println!("=========================================================================");
			self.slog.new_step(self.curr_step);
			// println!("== {}",self.base[0].term.total_size());
			i = i + 1;
			if self.bstack.is_empty(){
				println!("\nResult: Refuted");
				return SolverResult{t: SolverResultType::Refuted, steps: i};
			}
			let res = self.find_answer_global();
			match res{
				AnswerOption::Success(aid) => {
					self.transform(aid);
					self.curr_step = self.curr_step + 1;					
				},
				AnswerOption::Restart => {
					println!("restart");
				},
				AnswerOption::Fail => {
					println!("\nResult: Exhausted");
					return SolverResult{t: SolverResultType::Exhausted, steps: i};					
				},
				AnswerOption::Next => {
					panic!("");
				}
			}


			// if let Some(aid) = self.find_answer_global(){
			// 	self.transform(aid);
			// 	self.curr_step = self.curr_step + 1;
			// }else{
			// 	println!("\nResult: Exhausted");
			// 	return SolverResult{t: SolverResultType::Exhausted, steps: i};
			// }
			self.print();
		}
		println!("\nResult: LimitReached");
		return SolverResult{t: SolverResultType::LimitReached, steps: i};
	}
}




pub fn processing(tid:TermId, context: &Context, answer1: Option<&Answer>, env: &mut PEnv) -> ProcessingResult{
	let t = &env.psterms.get_term(&tid);
	match t{
		Term::AVariable(..) => {
			if let Some(new_tid) = context.get(&tid){
				ProcessingResult::Existing(*new_tid)
			}else if let Some(answer) = answer1{
				if let Some(new_tid) = answer.get(&tid){
					ProcessingResult::Existing(*new_tid)
				}else{
					panic!("");
				}
			}else{
				// println!("{}",
				// 	TidDisplay{
				// 		tid: tid,
				// 		psterms: env.psterms,
				// 		context: Some(context),
				// 		dm: DisplayMode::PlainSid,							
				// 	});	
				panic!("");
			}
		},
		Term::EVariable(..) => {
			if let Some(new_tid) = context.get(&tid){
				ProcessingResult::Existing(*new_tid)
			}else if let Some(answer) = answer1{
				if let Some(new_tid) = answer.get(&tid){
					ProcessingResult::Existing(*new_tid)
				}else{
					ProcessingResult::Existing(tid)
				}
			}else{
				ProcessingResult::Existing(tid)
			}			
		}
		Term::SConstant(..) | Term::Bool(..) | Term::Integer(..) | Term::String(..) => {
			ProcessingResult::Existing(tid)
		},
		Term::SFunctor(sid, args) => {
			let new_term = Term::SFunctor(
				*sid, 
				args
					.iter()
					.map(|arg| 
						processing(*arg, context, answer1, env).unwrap())
					.collect());
			env.psterms.get_tid(new_term)
		},
		Term::IFunctor(sid, args) => {
			let f = env.psterms.get_symbol(&sid).f.unwrap();
			processing(
				f(
					&args
						.iter()
						.map(|arg| 
							processing(*arg, context, answer1, env).unwrap())
						.collect(), 
					env),
				context,
				answer1,
				env)
		},
		Term::List(args) => {
			let new_term = Term::List(
				args
					.iter()
					.map(|arg| 
						processing(*arg, context, answer1, env).unwrap())
					.collect());
			env.psterms.get_tid(new_term)
		},		
	}
}


pub fn matching(
	btid:TermId, 
	qtid:TermId, 
	context: &Context, 
	curr_answer: &mut Answer, 
	psterms: &mut PSTerms, 
	base: &mut Base,
	attributes: &mut Attributes,
	bid: BlockId) -> bool{
	
	if btid == qtid{
		true
	}else{
		let bterm = psterms.get_term(&btid);
		let qterm = psterms.get_term(&qtid);
		match qterm{
			Term::AVariable(..) => {
				if let Some(new_qtid) = context.get(&qtid){
					matching(btid, *new_qtid, context, curr_answer, psterms, base, attributes, bid)
				}else if let Some(new_qtid) = curr_answer.get(&qtid){
					matching(btid, *new_qtid, context, curr_answer, psterms, base, attributes, bid)
				}else{
					curr_answer.push(qtid, btid);
					true
				}
			},
			Term::EVariable(..) => {
				if let Some(new_qtid) = context.get(&qtid){
					matching(btid, *new_qtid, context, curr_answer, psterms, base, attributes, bid)
				}else{
					false
				}
			},
			Term::SFunctor(q_sid, q_args) => {
				match bterm{
					Term::SFunctor(b_sid,b_args) if q_sid == b_sid => {
						q_args.iter().zip(b_args.iter()).all(|pair| matching(*pair.1, *pair.0, context, curr_answer, psterms, base, attributes, bid))
					},
					_ => false,
				}
			},
			Term::IFunctor(..) => {
				let p = psterms.len();
				
				let mut env = PEnv{
					psterms: psterms,
					base: base,
					answer: &curr_answer,
					attributes: attributes,
					bid: bid,
				};	
							
				let new_qtid = processing(qtid, context, Some(&curr_answer), &mut env).unwrap();
				let m = matching(btid, new_qtid, context, curr_answer, psterms, base, attributes, bid);
				psterms.back_to(p);
				m
			},
			Term::List(q_args) => {
				match bterm{
					Term::List(b_args) if q_args.len() == b_args.len() => {
						q_args.iter().zip(b_args.iter()).all(|pair| matching(*pair.1, *pair.0, context, curr_answer, psterms, base, attributes, bid))
					},
					_ => false,
				}
			},
			_ => {
				false
			}
		}
	}
}


