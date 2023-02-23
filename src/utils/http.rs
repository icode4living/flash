/*HTTP header parser checks if the request header matches the 
http specification
 */
use std::{fs, collections::{HashMap, hash_map::RandomState}, hash::Hash, fmt::format};
extern crate pest;
//#[macro_use]
use pest_derive::Parser;

use pest::{Parser, iterators::Pair};
//use pest::iterators::Pair;
#[derive(Parser)]
#[grammar = "grammars/http.pest"]
pub struct HttpParser;

//This enum represents the the http method
#[derive(Debug)]
pub enum HttpMethod {
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

pub fn parse_http(input: String)->Http{
    let file = HttpParser::parse(Rule::http, &input)
    .expect("unssuccessful parse").next().unwrap();
    let mut header: HashMap<String, String> = HashMap::new();
 // let h = header.entry("".to_string()).or_default();
  //header.insert("k".to_string(), "".to_string());
    let mut result = Http{method:"".to_string(), 
    uri: "".to_string(), version:"".to_string(),headers: header};
   // println!("{:#?}", file.into_inner());
    for line in file.into_inner(){

        match line.as_rule(){
            Rule::request=>{
                let mut inner_r = line.into_inner();
                result.method = inner_r.next().unwrap().as_str().to_string();

                result.uri =inner_r.next().unwrap().as_str().to_string();
                result.version =inner_r.next().unwrap().as_str().to_string();
                let header_name = inner_r.next().unwrap().as_str();
                let header_val = inner_r.next().unwrap().as_str();
           //let h =   head.insert(header_name.to_string(), header_val.to_string());
              header.insert(header_name.to_string(), header_val.to_string());


            },
            Rule::EOI=>(),
            Rule::delimiter=>(),
            Rule::header=>(),
            Rule::header_name=>(),
            Rule::header_value=>(),
            Rule::headers=>{
    
            },
            
            Rule::request_line=>{
    
            },
            Rule::method=>{
               let mut inner_rule = line.into_inner();
             // let nmethod = inner_rule.next().unwrap().as_str();
               result.method = inner_rule.next().unwrap().as_str().to_string();
    
            },
            Rule::uri=>{
    
            },
            Rule::version=>{
    
            },
            Rule::http=>{
    
            },
            Rule::whitespace=>(),
    
    
        }
    }

   //Http { method: line.into_inner()., uri: (), version: (), headers: () }
   //println!("{:#?}", line);

   result
   
   }
    



   // println!("{:#?}", file);

