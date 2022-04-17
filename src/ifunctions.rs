use crate::term::*;
use crate::answer::*;
use crate::misc::*;
use crate::solver::*;
use crate::environment::*;
use std::fs;

use std::collections::HashMap;

type IFunction = fn(&Vec<TermId>, &mut PEnv) -> TermId;

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

fn remove(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 1{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);

	let i = if let Term::Integer(i) = arg0{
		i
	}else{
		panic!("");
	};

	let b = if let LogItem::Matching{batom_i: b, ..} = env.answer.log[i as usize]{
		b
	}else{
		panic!("");
	};

	if let Some(bt) = env.base.get_mut(b){
		bt.deleted = true;
	}else{
		panic!("");
	}
	env.psterms.get_tid(Term::Bool(true)).unwrap()	
}

fn read_file_to_string(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 1{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);

	let n1 = if let Term::String(_n1) = arg0{
		_n1
	}else{
		panic!("");
	};
	
	let res = fs::read_to_string(&n1)
        .expect("Something went wrong reading the file");

	env.psterms.get_tid(Term::String(res)).unwrap()
}

fn solve(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 1{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);

	let n1 = if let Term::String(_n1) = arg0{
		_n1
	}else{
		panic!("");
	};
	

    let mut solver = Solver::parse_string(&n1);
    let res = solver.solver_loop(150);
    let r = if SolverResultType::Refuted == res.t{
    	true
    }else{
    	false
    };

	env.psterms.get_tid(Term::Bool(r)).unwrap()
}

fn string(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 1{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);

	let res = match arg0{
		Term::Integer(i) => i.to_string(),
		_ => "hello".to_string()
	};

	env.psterms.get_tid(Term::String(res)).unwrap()
}


fn notequal(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 2{
		panic!("");
	}
	let res = args[0] == args[1];

	env.psterms.get_tid(Term::Bool(!res)).unwrap()
}



// ====
pub fn init() -> (PSTerms, HashMap<String, SymbolId>){
	let mut psterms = PSTerms::new();
	let mut fmap = HashMap::new();


	let fs = HashMap::from([
		("+".to_string(), (ifunction_binary_integers!(plus, i64) as IFunction, Position::Infix)),
		("-".to_string(), (ifunction_binary_integers!(minus, i64) as IFunction, Position::Infix)),
		("*".to_string(), (ifunction_binary_integers!(multiply, i64) as IFunction, Position::Infix)),
		("==".to_string(), (ifunction_binary_integers!(eq, bool) as IFunction, Position::Infix)),
		("!=".to_string(), (ifunction_binary_integers!(noteq, bool) as IFunction, Position::Infix)),
		("<".to_string(), (ifunction_binary_integers!(lt, bool) as IFunction, Position::Infix)),
		(">".to_string(), (ifunction_binary_integers!(gt, bool) as IFunction, Position::Infix)),
		("<=".to_string(), (ifunction_binary_integers!(lteq, bool) as IFunction, Position::Infix)),
		(">=".to_string(), (ifunction_binary_integers!(gteq, bool) as IFunction, Position::Infix)),
		("++".to_string(), (concat as IFunction, Position::Infix)),
		("replace".to_string(), (replace as IFunction, Position::Classic)),
		("blen".to_string(), (blen as IFunction, Position::Classic)),
		("remove".to_string(), (remove as IFunction, Position::Classic)),
		("rfts".to_string(), (read_file_to_string as IFunction, Position::Classic)),
		("solve".to_string(), (solve as IFunction, Position::Classic)),
		("string".to_string(), (string as IFunction, Position::Classic)),
		("&".to_string(), (notequal as IFunction, Position::Infix)),
	]);


	for f in fs{
		let sid = psterms.add_ifunction(f.0.to_string(), Some((f.1).0), (f.1).1);
		fmap.insert(f.0, sid);
	}

	(psterms, fmap)
}