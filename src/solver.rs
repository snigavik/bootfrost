
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::misc::*;
use crate::term::*;
use crate::question::*;
use crate::context::*;
use crate::answer::*;
use crate::plain::*;
use crate::strategies::*;



struct FBlock{
	qid: QuestionId,
	aid: AnswerId,
	eid: TqfId,
	context: Context,
	pub bid: BlockId,
	pub activated: bool,
}

pub struct Solver{
	psterms: PSTerms,
	base: Vec<BTerm>,
	base_index: HashMap<TermId, usize>,
	tqfs: Vec<Tqf>,
	questions: Vec<Question>,
	stack: Vec<FBlock>,
	bid: BlockId,
	step: usize,
}

impl Solver{

	pub fn print_term(&self, tid: TermId, context: &Context){
		if let Some(new_tid) = context.get(&tid){
			self.print_term(*new_tid, context);
		}else{
			let t = self.psterms.get_term(&tid);
			match t{
				Term::AVariable(sid) => {
					let s = self.psterms.get_symbol(&sid);
					print!("{}.{}", s.name, s.uid);
				},
				Term::EVariable(sid, bid) => {
					let s = self.psterms.get_symbol(&sid);
					print!("{}.{}.{}", s.name, s.uid, bid.0);
				},
				Term::SConstant(sid) => {
					let s = self.psterms.get_symbol(&sid);
					print!("{}", s.name);
				},
				Term::Bool(b) => {
					print!("{}",b);
				},
				Term::Integer(i) => {
					print!("{}",i);
				},
				Term::String(s) => {
					print!("{}",s);
				},
				Term::SFunctor(sid, args) | Term::IFunctor(sid, args) => {
					let s = self.psterms.get_symbol(&sid);
					print!("{}", s.name);
					print!("(");
					for (i,a) in args.iter().enumerate(){
						self.print_term(*a,context);
						if i < args.len() - 1{
							print!(",");
						}
					}
					print!(")");
				}
			}
		}
	}

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
		for (i,b) in self.base.iter().enumerate(){
			self.print_term(b.term, &Context::new_empty());
			if i < self.base.len() - 1{
				print!(",");
			}			
		}
		println!("");
		for q in &self.questions{
			self.print_tqf(q.aformula, "".to_string(), &self.stack[q.fstack_i].context);
		}
	}

	pub fn parse(path: &str) -> Solver{
		let pf = crate::parser::parse_file(path);
		//let mut psterms = PSTerms::new();
		let mut vstack = vec![];
		let mut smap = HashMap::from([("false".to_string(),TermId(0)), ("true".to_string(),TermId(1))]);

		// let mut fmap = HashMap::new();
		let mut tqfs = vec![];
		let (mut psterms, mut fmap) = crate::ifunctions::init();

		let tid = plain_to_tqf(pf, &mut psterms, &mut vstack, &mut smap, &mut fmap, &mut tqfs);

		let mut fblocks: Vec<FBlock> = tqfs[tid.0].next.iter().enumerate().map(|(i,eid)|
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
			base: vec![],
			base_index: HashMap::new(),
			tqfs: tqfs,
			questions: vec![],
			stack: fblocks,
			bid: BlockId(bid),
			step:0,
		};

		solver.activate_top_block();
		solver
	}

	fn level(&self) -> usize{
		self.stack.iter().filter(|x| x.activated).count()
	}

	fn question_mut(&mut self, i:QuestionId) -> &mut Question{
		if let Some(q) = self.questions.get_mut(i.0){
			q
		}else{
			panic!("");
		}
	}

	fn tqf(&self, i: TqfId) -> &Tqf{
		if let Some(tqf) = self.tqfs.get(i.0){
			tqf
		}else{
			panic!("")
		}
	}

	fn next_a(&mut self, qid: QuestionId) -> bool{
		let mut question = self.questions.get_mut(qid.0).unwrap();
		let state_len = question.curr_answer_stack.last_mut().unwrap().len();
		let conj_len = question.curr_answer_stack.last_mut().unwrap().conj_len;
		if state_len < conj_len{
			let x = &self.tqfs[question.aformula.0].conj[state_len];
			let q_term = self.psterms.get_term(x);
			question.curr_answer_stack.last_mut().unwrap().state = MatchingState::Ready;
			match q_term{
				Term::SFunctor(..) => {
					if self.base.len() == 0{
						question.curr_answer_stack.last_mut().unwrap().state = MatchingState::Exhausted;
						false
					}else{
						let b = question.curr_answer_stack.last().unwrap().get_b(state_len);
						question.curr_answer_stack.last_mut().unwrap().push_satom(state_len, b);
						true
					}
				},
				Term::IFunctor(..) => {
					question.curr_answer_stack.last_mut().unwrap().push_iatom(state_len);
					true
				},
				_ => {
					panic!("");
				}
			}
		}else{
			question.curr_answer_stack.last_mut().unwrap().state = MatchingState::Answer;
			false
		}	
	}

	fn next_b(&mut self, qid: QuestionId){
		let mut question = self.questions.get_mut(qid.0).unwrap();
		question.curr_answer_stack.last_mut().unwrap().next_b();
	}

	fn next_k(&mut self, qid: QuestionId){
		let mut question = self.questions.get_mut(qid.0).unwrap();
		question.curr_answer_stack.last_mut().unwrap().next_k();
	}

	fn next_bounds(&mut self, qid: QuestionId) -> bool{
		let mut question = self.questions.get_mut(qid.0).unwrap();
		let blen = self.base.len();
		question.curr_answer_stack.last_mut().unwrap().shift_bounds(blen)
	}

	fn find_answer_local(&mut self, si: &StrategyItem, bid: BlockId) -> Option<Answer>{
		let qid = si.qid;
		dbg!(si.qid.0);
		let limit = si.limit;
		if let Some(top) = self.questions[qid.0].curr_answer_stack.last(){
			if top.bid != bid{
				let mut new_top = top.clone();
				new_top.bid = bid;
				self.questions[qid.0].curr_answer_stack.push(new_top);
			}
		}else{
			let new_top = Answer::new(bid, qid, self.base.len(), self.tqf(self.questions[qid.0].aformula).conj.len());
			self.questions[qid.0].curr_answer_stack.push(new_top);
		}


		let mut i = 0;
		while i < limit{
			let a = &self.questions[qid.0].curr_answer_stack.last().unwrap();
			i = i + 1;
			match &self.questions[qid.0].curr_answer_stack.last_mut().unwrap().state{
				MatchingState::Success | MatchingState::NextA | MatchingState::Zero => {
					self.next_a(qid);
					continue;
				},
				MatchingState::NextB | MatchingState::Fail => {
					self.next_b(qid);
					continue;
				},
				MatchingState::Ready => {
					let context = &self.stack[self.questions[qid.0].fstack_i].context;
					match self.questions[qid.0].curr_answer_stack.last().unwrap().last().unwrap(){
						LogItem::Matching{batom_i, qatom_i, ..} => {
							let bterm = &self.base[*batom_i];
							if bterm.deleted{
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Fail;
								continue;
							}
							let btid = bterm.term;
							let qtid = self.tqf(self.questions[qid.0].aformula).conj[*qatom_i];
							
							let mut curr_answer = &mut self.questions.get_mut(qid.0).unwrap().curr_answer_stack.last_mut().unwrap();
							if matching(&mut self.psterms, btid, qtid, context, curr_answer){
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Success;
								continue;
							}else{
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Fail;
								continue;								
							}
						},
						LogItem::Interpretation{qatom_i} => {
														
							let qtid = self.tqf(self.questions[qid.0].aformula).conj[*qatom_i];
							let curr_answer = &self.questions.get_mut(qid.0).unwrap().curr_answer_stack.last_mut().unwrap();
							let b = processing(qtid, &mut self.psterms, context, Some(&curr_answer)).unwrap();
							if self.psterms.check_value(&b){
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Success;
							}else{
								self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Fail;
							}
							continue;
						},
					}
				},
				MatchingState::Rollback => {
					if self.questions[qid.0].curr_answer_stack.last_mut().unwrap().len() > 0{
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::NextB;
						continue;
					}else{
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::NextK;
						continue;
					}
				},
				MatchingState::NextK => {
					self.next_k(qid);
				},
				MatchingState::Exhausted => {
					if self.next_bounds(qid){
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Zero;
					}else{
						break;
					}
				},
				MatchingState::Answer => {
					let nq =self.questions[qid.0].curr_answer_stack.last_mut().unwrap().clone();
					self.questions.get_mut(qid.0).unwrap().answers.push(nq);

					if self.questions[qid.0].curr_answer_stack.last_mut().unwrap().conj_len == 0{
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::Empty;	
					}else{
						self.question_mut(qid).curr_answer_stack.last_mut().unwrap().state = MatchingState::NextB;
					}

					let mut answer1 = self.questions[qid.0].answers.last().unwrap().clone();
					match si.selector{
						SelectorStrategy::First(f) => {
							if f(&answer1, &self.psterms){
								answer1.level = Some(self.stack.iter().filter(|x| x.activated).count());
								self.questions.get_mut(qid.0).unwrap().used_answers.push(answer1.clone());
								return Some(answer1)
							}else{
								continue;
							}
						},
						SelectorStrategy::Best => {
							continue;
						}
					}
				},
				MatchingState::Empty => {
					break;
				}
			}

		}
		None //		 
	}

	fn strategy(&self) -> Vec<StrategyItem>{		
		// let mut vq:Vec<StrategyItem> = 
		// self.questions
		// 	.iter()
		// 	.enumerate()
		// 	.map(|(i,q)| 
		// 		StrategyItem{
		// 			qid: QuestionId(i),
		// 			selector: SelectorStrategy::First(|x,y| true),
		// 			sf: StartFrom::Last,
		// 			limit:1000}).collect();
		// vq.rotate_left(self.step % self.questions.len());	
		// vq
		plain_shift_strategy(&self.questions, self.step)
	}

	fn find_answer_global(&mut self) -> Option<Answer>{
		let bid = self.stack.last().unwrap().bid;
		let strategy = self.strategy();
		for si in strategy.iter(){
			if let Some(answer) = self.find_answer_local(si, bid){
				println!("{}", AnswerDisplay{answer: &answer, psterms: &self.psterms, dm: DisplayMode::Plain});
				return Some(answer);
			}
		}
		None
	}

	fn remove_top_block(&mut self){
		if let Some(top) = self.stack.pop(){
			if top.activated{
				while let Some(last) = self.base.last(){
					if last.bid == top.bid{
						if let Some(bt) = self.base.pop(){
							self.base_index.remove(&bt.term);
						}else{
							panic!("");
						}
					}else{
						break;
					}
				}

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


	fn transform(&mut self, answer: Answer){
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
		commands.iter().for_each(|c| run_command(&mut self.psterms, *c));
		

		let mut new_blocks: Vec<FBlock> = 
			e_tqfs
				.iter()
				.enumerate()
				.map(|(i,ef)|
					FBlock{
						qid: qid,
						aid: AnswerId(qid.0, 100000),
						eid: *ef,
						context: Context::new(&curr_context, &answer, &self.tqfs[ef.0].vars, &mut self.psterms, origin_bid),
						bid: BlockId(self.bid.0 + i),
						activated: false,
					}).collect();
		self.stack.append(&mut new_blocks);
		//self.step = self.step + 1;

		self.activate_top_block_loop();
		// while self.stack.len() > 0{
		// 	if !self.activate_top_block(){
		// 		self.remove_solved_blocks();
		// 	}else{
		// 		return;
		// 	}	 
		// }
	}

	fn activate_top_block(&mut self) -> bool{
		let fstack_i = self.stack.len() - 1;
		if let Some(top) = self.stack.last_mut(){
			top.activated = true;
			let e_tqf = &self.tqfs[top.eid.0];
			let e_conj = &e_tqf.conj;
			let new_conj = e_conj.iter().map(|a| processing(*a, &mut self.psterms, &top.context, None).unwrap());
			for a in new_conj{
				if a == TermId(0){
					return false;
				}

				if let Some(i) = self.base_index.get(&a){
					if self.base[*i].deleted{
						self.base_index.insert(a, self.base.len());
						self.base.push(BTerm{term: a, bid: top.bid, deleted: false})
					}
				}else{
					self.base_index.insert(a, self.base.len());
					self.base.push(BTerm{term: a, bid: top.bid, deleted: false})
				}
			}	

			// add questions
			let a_tqfs = &e_tqf.next;
			let mut new_questions = 
				a_tqfs
					.iter()
					.map(|af| 
						Question{
							bid: top.bid,
							aformula: *af,
							fstack_i: fstack_i,
							curr_answer_stack: vec![],
							answers: vec![],
							used_answers: vec![],
						}).collect();
			self.questions.append(&mut new_questions);
			//top.activated = true;
			return true;		
		}else{
			panic!("");
		}
	}

	pub fn solver_loop(&mut self, limit:usize){
		let mut i = 0;
		while i < limit{
			println!("================================ Step {} ================================", self.step);
			i = i + 1;
			//dbg!(&self.psterms);
			if self.stack.is_empty(){
				println!("Refuted");
				break;
			}
			if let Some(answer) = self.find_answer_global(){
				self.transform(answer);
				self.step = self.step + 1;
			}else{
				println!("Exhausted");
				break;
			}
			self.print();
		}
	}
}


fn processing(tid:TermId, psterms: &mut PSTerms, context: &Context, answer1: Option<&Answer>) -> ProcessingResult{
	let t = &psterms.get_term(&tid);
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
						processing(*arg, psterms, context, answer1).unwrap())
					.collect());
			psterms.get_tid(new_term)
		},
		Term::IFunctor(sid, args) => {
			let f = psterms.get_symbol(&sid).f.unwrap();
			processing(
				f(
					&args
						.iter()
						.map(|arg| 
							processing(*arg, psterms, context, answer1).unwrap())
						.collect(), 
					psterms),
				psterms,
				context,
				answer1)
		},
	}
}


fn run_command(psterms: &mut PSTerms,  tid:TermId){

}

// 

fn matching(psterms: &mut PSTerms, btid:TermId, qtid:TermId, context: &Context, curr_answer: &mut Answer) -> bool{
	if btid == qtid{
		true
	}else{
		let bterm = psterms.get_term(&btid);
		let qterm = psterms.get_term(&qtid);
		match qterm{
			Term::AVariable(..) => {
				if let Some(new_qtid) = context.get(&qtid){
					matching(psterms, btid, *new_qtid, context, curr_answer)
				}else if let Some(new_qtid) = curr_answer.get(&qtid){
					matching(psterms, btid, *new_qtid, context, curr_answer)
				}else{
					curr_answer.push(qtid, btid);
					true
				}
			},
			Term::EVariable(..) => {
				if let Some(new_qtid) = context.get(&qtid){
					matching(psterms, btid, *new_qtid, context, curr_answer)
				}else{
					false
				}
			},
			Term::SFunctor(q_sid, q_args) => {
				match bterm{
					Term::SFunctor(b_sid,b_args) if q_sid == b_sid => {
						q_args.iter().zip(b_args.iter()).all(|pair| matching(psterms, *pair.1, *pair.0, context, curr_answer))
					},
					_ => false,
				}
			},
			Term::IFunctor(q_sid, q_args) => {
				let p = psterms.len();
				let new_qtid = processing(qtid, psterms, context, Some(&curr_answer)).unwrap();
				let m = matching(psterms, btid, new_qtid, context, curr_answer);
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


