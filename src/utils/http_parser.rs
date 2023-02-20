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
  let req =  my_value.entry("request".to_owned()).or_default();

 

    let item = HTTPParser::parse(Rule::http, &item)
    .expect("unsuccessful parse")
    .next().unwrap();
    for line in item.into_inner(){
      match line.as_rule(){
        Rule::request=>{
          let mut inner_rules = line.into_inner();
          req.insert("method".to_owned(), inner_rules.next().unwrap().as_str().to_owned());
          req.insert("path".to_owned(), inner_rules.next().unwrap().as_str().to_owned());
          req.insert("version".to_owned(), inner_rules.next().unwrap().as_str().to_owned());
         
         for nline in inner_rules{
            let mut inner_rule = nline.into_inner();
            let name = inner_rule.next().unwrap().as_str().to_owned();
            let val = inner_rule.next().unwrap().as_str().to_owned();

            req.insert(name, val);
         }

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


