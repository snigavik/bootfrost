use crate::term::*;
use crate::misc::*;
use crate::environment::*;
use std::collections::HashMap;



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
		|args: &Vec<TermId>, env: &mut PEnv| -> TermId{
			if args.len() != 2{
				panic!("");
			}

			let arg0 = env.psterms.get_term(&args[0]);
			let arg1 = env.psterms.get_term(&args[1]);

			let (n1,n2) = if let (Term::Integer(_n1), Term::Integer(_n2)) = (arg0, arg1){
				(_n1, _n2)
			}else{
				panic!("");
			};

			env.psterms.get_tid(result_term!($f(n1,n2), $tp)).unwrap()
		}
	}
}

fn concat(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 2{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);
	let arg1 = env.psterms.get_term(&args[1]);

	let (n1,n2) = if let (Term::String(_n1), Term::String(_n2)) = (arg0, arg1){
		(_n1, _n2)
	}else{
		panic!("");
	};

	let res = format!("{}{}",n1,n2);
	env.psterms.get_tid(Term::String(res)).unwrap()
}

fn replace(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 3{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);
	let arg1 = env.psterms.get_term(&args[1]);
	let arg2 = env.psterms.get_term(&args[2]);

	let (n1,n2,n3) = if let (Term::String(_n1), Term::String(_n2), Term::String(_n3)) = (arg0, arg1, arg2){
		(_n1, _n2, _n3)
	}else{
		panic!("");
	};
	
	let res = str::replace(&n1, &n2,&n3);
	env.psterms.get_tid(Term::String(res)).unwrap()
}

fn blen(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 0{
		panic!("");
	}	
	env.psterms.get_tid(Term::Integer(env.base.len().try_into().unwrap())).unwrap()	
}


// ====
pub fn init() -> (PSTerms, HashMap<String, SymbolId>){
	let mut psterms = PSTerms::new();
	let mut fmap = HashMap::new();


	let fs = HashMap::from([
		("+".to_string(), (ifunction_binary_integers!(plus, i64) as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		("-".to_string(), (ifunction_binary_integers!(minus, i64) as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		("*".to_string(), (ifunction_binary_integers!(multiply, i64) as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		("==".to_string(), (ifunction_binary_integers!(eq, bool) as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		("!=".to_string(), (ifunction_binary_integers!(noteq, bool) as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		("<".to_string(), (ifunction_binary_integers!(lt, bool) as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		(">".to_string(), (ifunction_binary_integers!(gt, bool) as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		("<=".to_string(), (ifunction_binary_integers!(lteq, bool) as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		(">=".to_string(), (ifunction_binary_integers!(gteq, bool) as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		("++".to_string(), (concat as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Infix)),
		("replace".to_string(), (replace as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Classic)),
		("blen".to_string(), (blen as fn(&Vec<TermId>, &mut PEnv) -> TermId, Position::Classic)),
	]);


	for f in fs{
		let sid = psterms.add_ifunction(f.0.to_string(), Some((f.1).0), (f.1).1);
		fmap.insert(f.0, sid);
	}

	(psterms, fmap)
}