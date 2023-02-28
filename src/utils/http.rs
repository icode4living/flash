/*HTTP header parser checks if the request header matches the 
http specification
 */
//use std::{fs, collections::{HashMap, hash_map::RandomState}, hash::Hash, fmt::format};
/*extern crate pest;
//#[macro_use]
use pest_derive::Parser;

use pest::{Parser, iterators::Pair};
//use pest::iterators::Pair;
#[derive(Parser)]
#[grammar = "grammars/http.pest"]
pub struct HttpParser;

//This enum represents the the http method
#[derive(Debug)]*/

extern crate pest;
//#[macro_use]
use pest_derive::Parser;

//extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammars/http.pest"]
struct HttpParser;
/*pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIIONS,
}
// the http header and request line data representation
#[derive(Debug)]
pub struct Http{
pub method: String,
pub uri: String,
pub version: String,
pub headers:HashMap<String, String>,

}
*/
pub fn parse_http(input: String){
    let parse_file = HttpParser::parse(Rule::file, &input)
    .unwrap().next().unwrap();

   // .expect("can not parse file")
for p in parse_file.into_inner(){
   // println!("{:#?}",p.into_inner())
    
match p.as_rule() {
    Rule::request=>{

    },
    Rule::method=>{
let mut t = p.into_inner();
println!("{:#?}",t.next().unwrap().as_str())
    },
    Rule::uri=>{
        let mut t = p.into_inner();
        println!("{:#?}",t.next().unwrap().as_str())
    },
    Rule::header=>{
        let mut t = p.into_inner();
        println!("{:#?}",t.next().unwrap().as_str())
    },
    Rule::header_name=>{
        let mut t = p.into_inner();
        println!("{:#?}",t.next().unwrap().as_str())
    },
    Rule::header_value=>{
        let mut t = p.into_inner();
        println!("{:#?}",t.next().unwrap().as_str())
    },
    Rule::version=>{
        let mut t = p.into_inner();
        println!("{:#?}",t.next().unwrap().as_str())
    }
    Rule::body=>{
        let mut t = p.into_inner();
        println!("{:#?}",t.next().unwrap().as_str())
    },
    Rule::EOI=>{
       
    },
    Rule::delimiter=>{
      
    },
    Rule::file=>{
        let mut t = p.into_inner();
        println!("file: {:#?}",t.next().unwrap().as_str())
    },
    Rule::whitespace=>{
        let mut t = p.into_inner();
        println!("{:#?}",t.next().unwrap().as_str())
    },
    Rule::headers=>{
        let mut t = p.into_inner();
        println!("{:#?}",t.next().unwrap().as_str())
    },
    Rule::request_line=>{
        let mut t = p.into_inner();
        println!("{:#?}",t.next().unwrap().as_str())
    }
    
}
}



//println!("{:#?}", file);


}
   

