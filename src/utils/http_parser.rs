/*HTTP header parser checks if the request header matches the 
http specification
 */
use std::fs;
extern crate pest;
//#[macro_use]
use pest_derive::Parser;

use pest::Parser;
#[derive(Parser)]
#[grammar = "grammars/http.pest"]
pub struct HTTPParser;

//skeleton test file
pub fn htpp_parser(){
    let unpassed_file = fs::read_to_string("sample/test_file.http")
    .expect("file could not be opened");

let unsuccessful_parse = HTTPParser::parse(Rule::uri, &unpassed_file)
.expect("unsucceful parse").next().unwrap();
println!("{:?}", unsuccessful_parse);
}
