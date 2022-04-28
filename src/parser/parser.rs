
use crate::plain::*;



use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::fmt;


//https://docs.python.org/3/reference/lexical_analysis.html#indentation

pub fn align_modulo(n:i64, m: i64) -> i64{
    n + (m - (n % m))
}


pub struct PfLine{
	pub line: String,
	pub indent: i64,
}

impl PfLine{
	pub fn new(line: String) -> PfLine{
		let s = line.clone();
		let mut indent = 0;
        
        for c in line.chars(){
            if c == ' '{
                indent = indent + 1;
            }
            if c == '\t'{
                indent = align_modulo(indent, 8);
            }
            if c != ' ' && c != '\t' {
                break;
            }
        }
		PfLine{line:s, indent:indent}
	}
}

impl fmt::Display for PfLine{
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result{
		write!(fmt,"{}:{}",self.indent, self.line)
    }
}


fn proc(lines: &Vec<PfLine>, k: &mut usize, stack_indents: &mut Vec<i64>) -> Vec<PlainFormula>{
    let mut res = vec![];
    if *k >= lines.len(){
        return res;
    }

    while lines[*k].indent > *stack_indents.last().unwrap(){
        stack_indents.push(lines[*k].indent);
        dbg!(&lines[*k].line);
        let mut pf = crate::parser::tqfline::TqfLineParser::new().parse(&lines[*k].line).unwrap();
        *k = *k + 1;
        if *k >= lines.len(){
            res.push(pf);
            return res;
        }
        pf.next = proc(lines, k, stack_indents);
        
        res.push(pf);
        if *k >= lines.len(){
            return res;
        }
    }
    //dbg!(&stack_indents);
    if !stack_indents.contains(&lines[*k].indent){
        panic!("Indentation error");
    }

    stack_indents.pop();
    return res;
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn file_to_pflines(path: &str) -> Vec<PfLine>{
    let mut true_lines: Vec<PfLine> = vec![];
    let mut buff = String::new();
    let mut flag = false;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(origin_line) = line {
                prepare_lines_string(&origin_line, &mut true_lines, &mut buff, &mut flag);
            }
        }
        return true_lines;
    }else{
        panic!("");
    }
}

pub fn string_to_pflines(s: &str) -> Vec<PfLine>{
    let mut true_lines: Vec<PfLine> = vec![];
    let mut buff = String::new();
    let mut flag = false;

    for origin_line in s.lines(){
        prepare_lines_string(&origin_line, &mut true_lines, &mut buff, &mut flag);
    }

    return true_lines;
}

pub fn prepare_lines_string(
        origin_line: &str, 
        true_lines: &mut Vec<PfLine>, 
        buff: &mut String, 
        flag: &mut bool){

    let line0 = if let Some((s,_)) = origin_line.split_once("#"){
        s
    }else{
        &origin_line
    };


    let line1 = if line0.ends_with("~"){
        line0.trim_end_matches("~")
    }else{
        *flag = true;
        &line0
    };

    if line1.trim().is_empty(){
        return;
    }

    buff.push_str(line1);
    if *flag{
        let pfline = PfLine::new(buff.clone());
        true_lines.push(pfline);
        *buff = String::new();
        *flag = false;
    }

}


fn pflines_to_plainformula(pflines: Vec<PfLine>) -> PlainFormula{
    let mut k = 0;
    let mut stack_indents = vec![-1];
    let mut res = PlainFormula{quantifier:"!".to_string(), vars: vec![], conjunct: vec![], commands:vec![], next: vec![]};
    res.next = proc(&pflines, &mut k, &mut stack_indents);
    res
}


pub fn parse_string(s: &str) -> PlainFormula{
    let pflines = string_to_pflines(s);
    pflines_to_plainformula(pflines)
}


pub fn parse_file(path: &str) -> PlainFormula{
    let pflines = file_to_pflines(path);
    for x in pflines.iter(){
        println!("{}",x);
    }
    pflines_to_plainformula(pflines) 
}




//