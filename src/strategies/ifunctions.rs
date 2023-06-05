use crate::term::*;
use crate::answer::*;
use crate::misc::*;
use crate::solver::*;
use crate::strategies::environment::*;
use crate::strategies::attributes::*;
use crate::strategies::strategies::Strategy;
use std::fs;



use std::collections::HashMap;

type IFunction = fn(&Vec<TermId>, &mut PEnv) -> TermId;

fn plus(a:i64,b:i64) -> i64 {a + b}
fn minus(a:i64,b:i64) -> i64 {a - b}
fn multiply(a:i64,b:i64) -> i64 {a * b}

// fn eq(a:i64, b:i64) -> bool {a == b}
// fn noteq(a:i64, b:i64) -> bool {a != b}
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

// lists
fn push1(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 2{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);
	let arg1 = env.psterms.get_term(&args[1]);

	let mut list = if let Term::List(_n1) = arg0{
		_n1.clone()
	}else{
		panic!("");
	};

	list.push(args[1]);
	env.psterms.get_tid(Term::List(list)).unwrap()
}

// lists
fn last1(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 1{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);

	let list = if let Term::List(_n1) = arg0{
		_n1
	}else{
		panic!("");
	};

	let res = list.last().unwrap();
	return *res
}

// lists
fn first1(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 1{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);

	let list = if let Term::List(_n1) = arg0{
		_n1
	}else{
		panic!("");
	};

	let res = list.first().unwrap();
	return *res
}

//lists
fn notempty(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 1{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);

	let list = if let Term::List(_n1) = arg0{
		_n1
	}else{
		panic!("");
	};

	let res = !list.is_empty();

	env.psterms.get_tid(Term::Bool(res)).unwrap()
}

//lists
fn subseteq(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 2{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);
	let arg1 = env.psterms.get_term(&args[1]);

	let list = if let Term::List(_n1) = arg1{
		_n1
	}else{
		panic!("");
	};

	let res = if let Term::List(_n2) = arg0{
		_n2.iter().all(|x|list.contains(&x))
	}else{
		panic!("");
	};

	env.psterms.get_tid(Term::Bool(res)).unwrap()
}


//lists
fn inlist(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 2{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);
	let arg1 = env.psterms.get_term(&args[1]);

	let list = if let Term::List(_n1) = arg1{
		_n1
	}else{
		panic!("");
	};

	// let res = if let Term::List(_n2) = arg0{
	// 	_n2.iter().all(|x|list.contains(&x))
	// }else{
	// 	list.contains(&args[0])		
	// };

	let res = list.contains(&args[0]);

	env.psterms.get_tid(Term::Bool(res)).unwrap()
}

//lists
fn notinlist(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 2{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);
	let arg1 = env.psterms.get_term(&args[1]);

	let list = if let Term::List(_n1) = arg1{
		_n1
	}else{
		panic!("");
	};

	let res = !list.contains(&args[0]);

	env.psterms.get_tid(Term::Bool(res)).unwrap()
}


// lists
fn sortlist(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 1{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);

	let mut list = if let Term::List(_n1) = arg0{
		_n1.clone()
	}else{
		panic!("");
	};

	let res = if list.iter().all(|x| env.psterms.is_integer(x)){
		let mut list_i: Vec<i64> = list.iter().map(|x|
			if let Term::Integer(i) = env.psterms.get_term(x){
				i
			}else{
				panic!("")
			}
		).collect();

		list_i.sort();
		list_i.iter().map(|x|env.psterms.get_tid(Term::Integer(*x)).unwrap()).collect::<Vec<TermId>>()
	}else{
		panic!("");
	};

	// list.push(args[1]);
	env.psterms.get_tid(Term::List(res)).unwrap()
}




// string, lists
fn concat(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 2{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);
	let arg1 = env.psterms.get_term(&args[1]);

	match (arg0, arg1){
		(Term::String(n1), Term::String(n2)) => {
			let res = format!("{}{}",n1,n2);
			env.psterms.get_tid(Term::String(res)).unwrap()
		},
		(Term::List(n1), Term::List(n2)) => {
			let mut res = vec![];
			res.append(&mut n1.clone());
			res.append(&mut n2.clone());
			env.psterms.get_tid(Term::List(res)).unwrap()
		}
		_ => {
			panic!("");
		}
	}

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

fn base_to_string(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 0{
		panic!("");
	}	
	
	let bstr = env.base.base
		.iter()
		.enumerate()
		.filter(|(i,_)|!env.attributes.check(KeyObject::BaseIndex(*i), AttributeName("deleted".to_string()), AttributeValue("true".to_string())))
		.map(|(_,x)|
			TidDisplay{
				tid: x.term,
				psterms: &env.psterms,
				context: None,
				dm: DisplayMode::Plain,
			}.to_string()).collect::<Vec<String>>().join(",");
	env.psterms.get_tid(Term::String(bstr)).unwrap()	
}


fn remove_fact(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
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

	env.attributes.set_attribute(KeyObject::BaseIndex(b), AttributeName("deleted".to_string()), AttributeValue("true".to_string()), env.bid);

	env.psterms.get_tid(Term::Bool(true)).unwrap()	
}

pub fn print_batoms(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 0{
		panic!("");
	}

	let vector = env.answer.get_batoms();
	print!("Terms used in the base: ");
	for v in vector{
		if let Some(b) = v{
			print!("{}, ", TidDisplay{
				tid: env.base[b].term,
				psterms: env.psterms,
				context: None,
				dm: DisplayMode::Plain,
			});
		}else{
			print!("=i=");
		}
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
	if args.len() != 2{
		panic!("");
	}

	let arg0 = env.psterms.get_term(&args[0]);
	let arg1 = env.psterms.get_term(&args[1]);

	let n1 = if let Term::String(_n1) = arg0{
		_n1
	}else{
		panic!("");
	};
	
	let d = if let Term::Integer(_n2) = arg1{
		_n2
	}else{
		panic!("");
	};

	

    let mut solver = Solver::parse_string(&n1, Strategy::General);
    let res = solver.solver_loop(d.try_into().unwrap());
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


fn noteq(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 2{
		panic!("");
	}
	let res = args[0] == args[1];

	env.psterms.get_tid(Term::Bool(!res)).unwrap()
}

fn eq(args: &Vec<TermId>, env: &mut PEnv) -> TermId{
	if args.len() != 2{
		panic!("");
	}
	let res = args[0] == args[1];

	env.psterms.get_tid(Term::Bool(res)).unwrap()
}



// ====
pub fn init() -> (PSTerms, HashMap<String, SymbolId>){
	let mut psterms = PSTerms::new();
	let mut fmap = HashMap::new();


	let fs = HashMap::from([
		("!=".to_string(), (noteq as IFunction, Position::Infix)),
		("==".to_string(), (eq as IFunction, Position::Infix)),
		("+".to_string(), (ifunction_binary_integers!(plus, i64) as IFunction, Position::Infix)),
		("-".to_string(), (ifunction_binary_integers!(minus, i64) as IFunction, Position::Infix)),
		("*".to_string(), (ifunction_binary_integers!(multiply, i64) as IFunction, Position::Infix)),
		// ("==".to_string(), (ifunction_binary_integers!(eq, bool) as IFunction, Position::Infix)),
		// ("!=".to_string(), (ifunction_binary_integers!(noteq, bool) as IFunction, Position::Infix)),
		("<".to_string(), (ifunction_binary_integers!(lt, bool) as IFunction, Position::Infix)),
		(">".to_string(), (ifunction_binary_integers!(gt, bool) as IFunction, Position::Infix)),
		("<=".to_string(), (ifunction_binary_integers!(lteq, bool) as IFunction, Position::Infix)),
		(">=".to_string(), (ifunction_binary_integers!(gteq, bool) as IFunction, Position::Infix)),
		("++".to_string(), (concat as IFunction, Position::Infix)),
		("replace".to_string(), (replace as IFunction, Position::Classic)),
		("blen".to_string(), (blen as IFunction, Position::Classic)),
		("base_to_string".to_string(), (base_to_string as IFunction, Position::Classic)),
		("remove_fact".to_string(), (remove_fact as IFunction, Position::Classic)),
		("read_file_to_string".to_string(), (read_file_to_string as IFunction, Position::Classic)),
		("solve".to_string(), (solve as IFunction, Position::Classic)),
		("string".to_string(), (string as IFunction, Position::Classic)),
		("push".to_string(), (push1 as IFunction, Position::Classic)),
		("last".to_string(), (last1 as IFunction, Position::Classic)),
		("first".to_string(), (first1 as IFunction, Position::Classic)),
		("notempty".to_string(), (notempty as IFunction, Position::Classic)),
		("in".to_string(), (inlist as IFunction, Position::Infix)),
		("notin".to_string(), (notinlist as IFunction, Position::Infix)),
		("subseteq".to_string(), (subseteq as IFunction, Position::Infix)),
		("sort".to_string(), (sortlist as IFunction, Position::Infix)),
		// ("&".to_string(), (notequal as IFunction, Position::Infix)),
	]);


	for f in fs{
		let sid = psterms.add_ifunction(f.0.to_string(), Some((f.1).0), (f.1).1);
		fmap.insert(f.0, sid);
	}

	(psterms, fmap)
}