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
	pub indent: usize,
}

impl PfLine{
	pub fn new(line: String) -> PfLine{
		let mut s = line.clone();
		let mut indent = 0;
		while let Some(x) = s.strip_prefix("\t"){
			//let len1 = s.len();
			//s = s.trim_start_matches("\t").to_string();
			//let len2 = s.len();
			//if len1 == len2{
			//	break;
			//}
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


pub fn parse(){
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
            		true_lines.push(PfLine::new(buff.clone()));
            		buff = String::new();
            		flag = false;
            	}
                //println!("{}", line1);
            }
        }
        for x in true_lines.iter(){
        	println!("{}", x);
        	let tqf = crate::tqfline::TqfLineParser::new().parse(&x.line).unwrap();
        }	
    }	
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}






//