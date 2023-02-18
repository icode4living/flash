/*HTTP header parser checks if the request header matches the 
http specification
 */
use std::{fs, collections::{HashMap, hash_map::RandomState}, hash::Hash, fmt::format};
extern crate pest;
//#[macro_use]
use pest_derive::Parser;

use pest::Parser;
#[derive(Parser)]
#[grammar = "grammars/http.pest"]
pub struct HTTPParser;


pub fn store_http(input: &'static str)->HashMap<&'static str, HashMap<&'static str,&'static str>> {
    let mut my_value:HashMap<&str, HashMap<&str,&str>>= HashMap::new();
  //  my_value.insert("name", input);
    

   my_value
}


