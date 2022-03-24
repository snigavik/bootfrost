use crate::question::*;
use crate::term::*;
use crate::misc::*;
use crate::plain::*;


use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::fmt;


struct PfLine{
	pub line: String,
	pub indent: i64,
}

impl PfLine{
	pub fn new(line: String) -> PfLine{
		let mut s = line.clone();
		let mut indent = 0;
		while let Some(x) = s.strip_prefix("\t"){
			s = x.to_string();
			indent = indent + 1;
		}
		if s.starts_with(" "){
			panic!("");
		}

		PfLine{line:s, indent:indent}
	}
}

impl fmt::Display for PfLine{
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
		write!(fmt,"{}:{}",self.indent, self.line)
    }
}


fn proc(lines: &Vec<PfLine>, k: &mut usize, parent_indent: i64) -> Vec<PlainFormula>{
	let mut res = vec![];
	if *k >= lines.len(){
        return res;
    }
	while lines[*k].indent == parent_indent + 1{
		let mut pf = crate::tqfline::TqfLineParser::new().parse(&lines[*k].line).unwrap();
        let new_parent_indent = lines[*k].indent;
        *k = *k + 1;
        if *k >= lines.len(){
            res.push(pf);
            return res;
        }
		pf.next = proc(lines, k, new_parent_indent);
        if *k >= lines.len(){
            res.push(pf);
            return res;
        }
		res.push(pf);
	}
	return res;
}



pub fn parse() -> PlainFormula{
	let mut true_lines: Vec<PfLine> = vec![];
	let mut buff = String::new();

    if let Ok(lines) = read_lines("./formula.pcf") {
    	let mut flag = false;
        for line in lines {
            if let Ok(origin_line) = line {
            	let line0 = if let Some((s,_)) = origin_line.split_once("//"){
            		s
            	}else{
            		&origin_line
            	};


            	let line1 = if line0.ends_with("~"){
            		line0.trim_end_matches("~")
            	}else{
            		flag = true;
            		&line0
            	};
            	buff.push_str(line1);
            	if flag{
            		let pfline = PfLine::new(buff.clone());
            		if !pfline.line.trim_start().is_empty(){
            			true_lines.push(pfline);
            		}
            		buff = String::new();
            		flag = false;
            	}
                // println!("{}", line1);
            }
        }

        let mut k = 0;
        let mut res = PlainFormula{quantifier:"!".to_string(), vars: vec![], conjunct: vec![], commands:vec![], next: vec![]};
        res.next = proc(&true_lines, &mut k, -1);
        return res;	
    }else{
    	panic!("");
    }
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}






//