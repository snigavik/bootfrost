use crate::term::*;
use crate::misc::*;
use std::collections::HashMap;

use std::ops::Mul;

fn plus(a:i64,b:i64) -> i64 {a + b}
fn minus(a:i64,b:i64) -> i64 {a - b}
fn multiply(a:i64,b:i64) -> i64 {a * b}

fn eq(a:i64, b:i64) -> bool {a == b}
fn noteq(a:i64, b:i64) -> bool {a != b}
fn lt(a:i64, b:i64) -> bool {a < b}
fn gt(a:i64, b:i64) -> bool {a > b}
fn lteq(a:i64, b:i64) -> bool {a <= b}
fn gteq(a:i64, b:i64) -> bool {a >= b}


macro_rules! ifunction_1{
	($f:tt) => {
		|args: &Vec<TermId>, psterms: &mut PSTerms| -> TermId{
			if args.len() != 2{
				panic!("");
			}

			let arg0 = psterms.get_term(&args[0]);
			let arg1 = psterms.get_term(&args[1]);

			let (n1,n2) = if let (Term::Integer(_n1), Term::Integer(_n2)) = (arg0, arg1){
				(_n1, _n2)
			}else{
				panic!("");
			};

			psterms.get_tid(Term::Integer($f(n1,n2))).unwrap()
		}
	}
}

macro_rules! ifunction_cmp{
	($f:tt) => {
		|args: &Vec<TermId>, psterms: &mut PSTerms| -> TermId{
			if args.len() != 2{
				panic!("");
			}

			let arg0 = psterms.get_term(&args[0]);
			let arg1 = psterms.get_term(&args[1]);

			let (n1,n2) = if let (Term::Integer(_n1), Term::Integer(_n2)) = (arg0, arg1){
				(_n1, _n2)
			}else{
				panic!("");
			};

			psterms.get_tid(Term::Bool($f(n1,n2))).unwrap()
		}
	}
}

fn concat(args: &Vec<TermId>, psterms: &mut PSTerms) -> TermId{
	if args.len() != 2{
		panic!("");
	}

	let arg0 = psterms.get_term(&args[0]);
	let arg1 = psterms.get_term(&args[1]);

	let (n1,n2) = if let (Term::String(_n1), Term::String(_n2)) = (arg0, arg1){
		(_n1, _n2)
	}else{
		panic!("");
	};
	let mut res = n1.clone();
	res.push_str(&n2);
	psterms.get_tid(Term::String(res)).unwrap()
}




// ====
pub fn init() -> (PSTerms, HashMap<String, SymbolId>){
	let mut psterms = PSTerms::new();
	let mut fmap = HashMap::new();

	let fs = HashMap::from([
		("+".to_string(), (ifunction_1!(plus) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("-".to_string(), (ifunction_1!(minus) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("*".to_string(), (ifunction_1!(multiply) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("==".to_string(), (ifunction_cmp!(eq) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("!=".to_string(), (ifunction_cmp!(noteq) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("<".to_string(), (ifunction_cmp!(lt) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		(">".to_string(), (ifunction_cmp!(gt) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("<=".to_string(), (ifunction_cmp!(lteq) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		(">=".to_string(), (ifunction_cmp!(gteq) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("++".to_string(), (concat as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
	]);


	for f in fs{
		let sid = psterms.add_ifunction(f.0.to_string(), Some((f.1).0), (f.1).1);
		fmap.insert(f.0, sid);
	}

	(psterms, fmap)
}