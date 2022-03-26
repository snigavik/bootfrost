use crate::term::*;
use crate::misc::*;
use std::collections::HashMap;


//addition
fn add(args: &Vec<TermId>, psterms: &mut PSTerms) -> TermId{
	if args.len() != 2{
		panic!("");
	}

	let arg0 = psterms.get_term(&args[0]);
	let arg1 = psterms.get_term(&args[1]);

	let n1 = if let Term::Integer(_n1) = arg0{
		_n1
	}else{
		panic!("");
	};

	let n2 = if let Term::Integer(_n2) = arg1{
		_n2
	}else{
		panic!("");
	};

	let r = psterms.get_tid(Term::Integer(n1+n2));
	match r{
		ProcessingResult::Existing(t) => t,
		ProcessingResult::New(t) => t,	
		Error => {
			panic!("");
		}
	}

}

pub fn init() -> (PSTerms, HashMap<String, SymbolId>){
	let mut psterms = PSTerms::new();
	let mut fmap = HashMap::new();

	let mut fs = HashMap::from([
		("+".to_string(), add),
	]);


	for f in fs{
		let sid = psterms.add_ifunction(f.0.to_string(), Some(f.1));
		fmap.insert(f.0, sid);
	}

	(psterms, fmap)
}