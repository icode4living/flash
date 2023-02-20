/*HTTP header parser checks if the request header matches the 
http specification
 */
use std::{fs, collections::{HashMap, hash_map::RandomState}, hash::Hash, fmt::format};
extern crate pest;
//#[macro_use]
use pest_derive::Parser;

use pest::Parser;
//use pest::iterators::Pair;
#[derive(Parser)]
#[grammar = "grammars/http.pest"]
pub struct HTTPParser;


pub fn store_http(item: String)->HashMap<String,HashMap<String,String>>{
  let mut my_value:HashMap<String,HashMap<String,String>> = HashMap::new();

 // my_value.insert("test", "test");
   //let mut req = "";

    let item = HTTPParser::parse(Rule::http, &item)
    .expect("unsuccessful parse")
    .next().unwrap();
    for line in item.into_inner(){
      match line.as_rule(){
        Rule::request=>{
          let mut inner_rules = line.into_inner();
        //  req = inner_rules.next().unwrap().as_str();
          let nvalue = my_value.entry("request".to_owned()).or_default();
         nvalue.insert("method".to_owned(), inner_rules.next().unwrap().as_str().to_owned());
          nvalue.insert("path".to_owned(), inner_rules.next().unwrap().as_str().to_owned());
          nvalue.insert("version".to_owned(), inner_rules.next().unwrap().as_str().to_owned());
          nvalue.insert("cookie".to_owned(), inner_rules.next().unwrap().as_str().to_owned());


         //my_value.insert("method".to_owned(), method.to_owned());

        }
        Rule::request_line=>{
        
        }
        Rule::uri=>{

        }
        Rule::method=>{

        }
        Rule::version =>{

        }
        Rule::whitespace=>{

        }
        Rule::headers=>{

        }
        Rule::header=>{

        }
        Rule::header_name=>{

        }
        Rule::header_value=>{

        }
        Rule::delimiter=>{

        }
        Rule::http=>{

        }

        Rule::EOI=>()
      }
      
    }
     // my_value
   //println!("{:?}", my_value)
   my_value

}


