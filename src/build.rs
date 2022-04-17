extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
    
    // lalrpop::Configuration::new()
    // 	.set_out_dir("./")
    // 	.process_dir("./src/parser/")
    // 	.unwrap();
	
	// lalrpop::Configuration::new()
 //        .generate_in_source_tree()
 //        .process();

}