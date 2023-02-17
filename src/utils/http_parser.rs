/*HTTP header parser checks if the request header matches the 
http specification
 */
use std::{fs, collections::{HashMap, hash_map::RandomState}, hash::Hash};
extern crate pest;
//#[macro_use]
use pest_derive::Parser;

use pest::Parser;
#[derive(Parser)]
#[grammar = "grammars/http.pest"]
pub struct HTTPParser;


//Hash map for storing parsed http header value
pub fn parse_http<T> (item:String)-> HashMap<T, HashMap<T,T>>
where T: Eq,
    T: Hash,
    T: ToString   
{
    let mut header: HashMap<T, HashMap<T,T>> = HashMap::new();
    header.insert("sam", "Afo");
   // header.insert(&"name", &"sam");
    header
}
