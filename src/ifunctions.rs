use crate::term::*;
use crate::misc::*;
use std::collections::HashMap;

fn plus(args: &Vec<TermId>, psterms: &mut PSTerms) -> TermId{
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

	psterms.get_tid(Term::Integer(n1+n2)).unwrap()
}

fn minus(args: &Vec<TermId>, psterms: &mut PSTerms) -> TermId{
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

	psterms.get_tid(Term::Integer(n1-n2)).unwrap()
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
		("+".to_string(), (plus, Position::Infix))
		// ("-".to_string(), minus),
		// ("++".to_string(), concat),
	]);


	for f in fs{
		let sid = psterms.add_ifunction(f.0.to_string(), Some((f.1).0), (f.1).1);
		fmap.insert(f.0, sid);
	}

	(psterms, fmap)
}