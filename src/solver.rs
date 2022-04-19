
use std::collections::HashMap;


use crate::misc::*;
use crate::term::*;
use crate::question::*;
use crate::context::*;
use crate::answer::*;
use crate::plain::*;
use crate::strategies::*;
use crate::base::*;
use crate::environment::*;

#[derive(Debug, Eq, PartialEq)]
pub enum SolverResultType{
	Refuted,
	Exhausted,
	LimitReached,
}

pub struct SolverResult{
	pub t: SolverResultType,
	pub steps: usize,
}

struct BranchBlock{
	pub aid: AnswerId,
	pub eindex: usize,
	pub context: Context,
	pub bid: BlockId,
	pub enabled: bool,
}

pub struct FBlock{
	qid: QuestionId,
	aid: AnswerId,
	eid: TqfId,
	pub context: Context,
	pub bid: BlockId,
	pub activated: bool,
}

pub struct Solver{
	psterms: PSTerms,
	base: Base,
	tqfs: Vec<Tqf>,
	questions: Vec<Question>,
	stack: Vec<FBlock>,
	bstack: Vec<BranchBlock>,
	bid: BlockId,
	step: usize,
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
			dm: DisplayMode::Plain,
			d: ",",
		});

		// conj
		print!(" ");
		print!("{}", TidsDisplay{
			tids: &tqf.conj,
			psterms: &self.psterms,
			context: Some(context),
			dm: DisplayMode::Plain,
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
				dm: DisplayMode::Plain,
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

	pub fn print(&self){
		for (i,b) in self.base.base.iter().enumerate(){
			if b.deleted{ print!("[");}

			print!("{}", TidDisplay{
				tid: b.term,
				psterms: &self.psterms,
				context: None,
				dm: DisplayMode::Plain,
			});	


			if b.deleted{ print!("]");}			
			if i < self.base.len() - 1{ print!(",");}
			
		}
		println!("\n");
		for q in &self.questions{
			self.print_tqf(q.aformula, "".to_string(), &self.stack[q.fstack_i].context);
		}
	}

	pub fn parse_file(path: &str) -> Solver{
		let pf = crate::parser::parser::parse_file(path);
		Solver::from_pf(pf)
	}

	pub fn parse_string(s: &str) -> Solver{
		let pf = crate::parser::parser::parse_string(s);
		Solver::from_pf(pf)
	}

	pub fn from_pf(pf: PlainFormula) -> Solver{

		let mut vstack = vec![];
		let mut smap = HashMap::from([("false".to_string(),TermId(0)), ("true".to_string(),TermId(1))]);

		let mut tqfs = vec![];
		let (mut psterms, mut fmap) = crate::ifunctions::init();

		let tid = plain_to_tqf(pf, &mut psterms, &mut vstack, &mut smap, &mut fmap, &mut tqfs);

		let fblocks: Vec<FBlock> = tqfs[tid.0].next.iter().enumerate().map(|(i,eid)|
			FBlock{
				qid:QuestionId(1000000000), 
				aid: AnswerId(1000000000, 1000000000),
				eid: *eid,
				context: Context::new_empty(),
				bid: BlockId(i),
				activated: false,
			}
		).collect();

		let bid = fblocks.len();


		let mut solver = Solver{
			psterms: psterms,
			base: Base::new(),
			tqfs: tqfs,
			questions: vec![],
			stack: fblocks,
			bstack: vec![],
			bid: BlockId(bid),
			step:0,
		};

		solver.activate_top_block();
		solver
	}

	fn level(&self) -> usize{
		self.stack.iter().filter(|x| x.activated).count()
	}


	fn strategy(&self) -> Vec<StrategyItem>{		
		
		//plain_shift_strategy(&self.questions, self.step);

		let curr_level = self.stack.iter().filter(|x| x.activated).count();
		general_strategy(&self.questions, &self.tqfs, curr_level)
	}

	fn find_answer_global(&mut self) -> Option<(Answer, usize)>{
		let bid = self.stack.last().unwrap().bid;
		let strategy = self.strategy();
		for si in strategy.iter(){
			if let Some((answer, aid_i)) = self.questions[si.qid.0].find_answer_local(si, bid, &mut self.psterms, &self.tqfs, &mut self.base, &self.stack){
				println!("{}: {}",si.qid.0, AnswerDisplay{answer: &answer, psterms: &self.psterms, dm: DisplayMode::Plain});
				return Some((answer, aid_i));
			}
		}
		None
	}

	fn remove_top_block(&mut self){
		if let Some(top) = self.stack.pop(){
			if top.activated{
				self.base.remove(top.bid);

				self.questions.retain(|q| q.bid != top.bid);

				self.questions.iter_mut().for_each(|q| q.remove_answers(top.bid));
			}else{
				panic!("");
			}
		}else{
			panic!("");
		}
	}

	fn remove_solved_blocks(&mut self){
		while let Some(top) = self.stack.last(){
			if top.activated{
				self.remove_top_block();
			}else{
				break;
			}
		}
	}

	fn activate_top_block_loop(&mut self){
		while self.stack.len() > 0{
			if !self.activate_top_block(){
				self.remove_solved_blocks();
			}else{
				return;
			}	 
		}		
	}


	fn activate_top_block(&mut self) -> bool{
		let fstack_i = self.stack.len() - 1;
		let level = self.level();
		if let Some(top) = self.stack.last_mut(){
			top.activated = true;
			let e_tqf = &self.tqfs[top.eid.0];
			let e_conj = &e_tqf.conj;
		

			let new_conj: Vec<TermId> = if level > 0{
				let mut env = PEnv{
					psterms: &mut self.psterms,
					base: &mut self.base,
					answer: &self.questions[top.qid.0].answers[top.aid.1],
				};				

				e_conj
					.iter()
					.map(|a| 
						processing(*a, &top.context, None, &mut env).unwrap())
					.collect()
			}else{
				e_conj.clone()
			};

			for a in new_conj{
				if a == TermId(0){
					return false;
				}

				self.base.push_and_check(a,top.bid);
			}	

			// add questions
			let a_tqfs = &e_tqf.next;
			let q_len = self.questions.len();
			let mut new_questions = 
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
			self.questions.append(&mut new_questions);
			return true;		
		}else{
			panic!("");
		}
	}


	fn transform(&mut self, answer: Answer, aid_i: usize){
		let qid = answer.qid;
		let curr_context = &self.stack[self.questions[qid.0].fstack_i].context;	
		let origin_bid = self.stack[self.questions[qid.0].fstack_i].bid;	
		let a_tqf = &self.questions[qid.0].aformula;
		let e_tqfs = &self.tqfs[a_tqf.0].next;

		if e_tqfs.len() == 0{
			self.remove_solved_blocks();
			self.activate_top_block_loop();
			return;
		}

		let commands = &self.tqfs[a_tqf.0].commands;

		let mut env = PEnv{
			psterms: &mut self.psterms,
			base: &mut self.base,
			answer: &answer,
		};
		commands.iter().for_each(|c| {processing(*c, &curr_context, Some(&answer), &mut env);});
		

		let mut new_blocks: Vec<FBlock> = 
			e_tqfs
				.iter()
				.enumerate()
				.map(|(i,ef)|
					FBlock{
						qid: qid,
						aid: AnswerId(qid.0, aid_i),
						eid: *ef,
						context: Context::new(&curr_context, &answer, &self.tqfs[ef.0].vars, &mut self.psterms, origin_bid),
						bid: BlockId(self.bid.0 + i),
						activated: false,
					}).collect();
		self.stack.append(&mut new_blocks);

		self.activate_top_block_loop();
	}



	// ================
	fn disable_block(&mut self){
		if let Some(top) = self.bstack.last_mut(){
			if top.enabled{
				self.base.remove(top.bid);

				self.questions.retain(|q| q.bid != top.bid);

				self.questions.iter_mut().for_each(|q| q.remove_answers(top.bid));

				top.enabled = false;
			}else{
				panic!("");
			}
		}else{
			panic!("");
		}
	}

	fn enable_block(&mut self) -> bool{
		let fstack_i = self.bstack.len() - 1;
		let level = self.bstack.len();
		if let Some(top) = self.bstack.last_mut(){
			top.enabled = true;
			let eid = &self.tqfs[self.questions[top.aid.0].aformula.0].next[top.eindex];
			let etqf = &self.tqfs[eid.0];
			let econj = &etqf.conj;
		

			let new_conj: Vec<TermId> = if level > 1{
				let mut env = PEnv{
					psterms: &mut self.psterms,
					base: &mut self.base,
					answer: &self.questions[top.aid.0].answers[top.aid.1],
				};				

				econj
					.iter()
					.map(|a| 
						processing(*a, &top.context, None, &mut env).unwrap())
					.collect()
			}else{
				econj.clone()
			};

			for a in new_conj{
				if a == TermId(0){
					return false;
				}

				self.base.push_and_check(a,top.bid);
			}	

			// add questions
			let a_tqfs = &etqf.next;
			let q_len = self.questions.len();
			let mut new_questions = 
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
			self.questions.append(&mut new_questions);
			return true;		
		}else{
			panic!("");
		}
		true
	}

	pub fn next_branch() -> bool{
		// aid: AnswerId,
		// eindex: usize,
		// pub context: Context,
		// pub bid: BlockId,
		true

	}



	pub fn solver_loop(&mut self, limit:usize) -> SolverResult{
		let mut i = 0;
		while i < limit{
			println!("================================ Step {}, stack: {}  ================================", self.step, self.stack.len());
			i = i + 1;
			//dbg!(&self.psterms);
			if self.stack.is_empty(){
				println!("Refuted");
				return SolverResult{t: SolverResultType::Refuted, steps: i};
			}
			if let Some((answer, aid_i)) = self.find_answer_global(){
				self.transform(answer, aid_i);
				self.step = self.step + 1;
			}else{
				println!("Exhausted");
				return SolverResult{t: SolverResultType::Exhausted, steps: i};
			}
			self.print();
		}
		println!("LimitReached");
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
	}
}


pub fn matching(
	btid:TermId, 
	qtid:TermId, 
	context: &Context, 
	curr_answer: &mut Answer, 
	psterms: &mut PSTerms, 
	base: &mut Base) -> bool{
	
	if btid == qtid{
		true
	}else{
		let bterm = psterms.get_term(&btid);
		let qterm = psterms.get_term(&qtid);
		match qterm{
			Term::AVariable(..) => {
				if let Some(new_qtid) = context.get(&qtid){
					matching(btid, *new_qtid, context, curr_answer, psterms, base)
				}else if let Some(new_qtid) = curr_answer.get(&qtid){
					matching(btid, *new_qtid, context, curr_answer, psterms, base)
				}else{
					curr_answer.push(qtid, btid);
					true
				}
			},
			Term::EVariable(..) => {
				if let Some(new_qtid) = context.get(&qtid){
					matching(btid, *new_qtid, context, curr_answer, psterms, base)
				}else{
					false
				}
			},
			Term::SFunctor(q_sid, q_args) => {
				match bterm{
					Term::SFunctor(b_sid,b_args) if q_sid == b_sid => {
						q_args.iter().zip(b_args.iter()).all(|pair| matching(*pair.1, *pair.0, context, curr_answer, psterms, base))
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
				};	
							
				let new_qtid = processing(qtid, context, Some(&curr_answer), &mut env).unwrap();
				let m = matching(btid, new_qtid, context, curr_answer, psterms, base);
				psterms.back_to(p);
				m
			},
			_ => {
				//panic!("");
				false
			}
		}
	}
}


