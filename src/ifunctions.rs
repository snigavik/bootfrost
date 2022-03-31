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


macro_rules! result_term{
	($res:expr, bool) => {
		Term::Bool($res)
	};
	($res:expr, i64) => {
		Term::Integer($res)
	}
}

macro_rules! ifunction_binary_integers{
	($f:tt, $tp:tt) => {
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

			psterms.get_tid(result_term!($f(n1,n2), $tp)).unwrap()
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
		("+".to_string(), (ifunction_binary_integers!(plus, i64) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("-".to_string(), (ifunction_binary_integers!(minus, i64) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("*".to_string(), (ifunction_binary_integers!(multiply, i64) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("==".to_string(), (ifunction_binary_integers!(eq, bool) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("!=".to_string(), (ifunction_binary_integers!(noteq, bool) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("<".to_string(), (ifunction_binary_integers!(lt, bool) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		(">".to_string(), (ifunction_binary_integers!(gt, bool) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("<=".to_string(), (ifunction_binary_integers!(lteq, bool) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		(">=".to_string(), (ifunction_binary_integers!(gteq, bool) as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
		("++".to_string(), (concat as fn(&Vec<TermId>, &mut PSTerms) -> TermId, Position::Infix)),
	]);


	for f in fs{
		let sid = psterms.add_ifunction(f.0.to_string(), Some((f.1).0), (f.1).1);
		fmap.insert(f.0, sid);
	}

	(psterms, fmap)
}