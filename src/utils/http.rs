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
pub method: HttpMethod,
pub uri: &'static str,
pub version: &'static str,
pub headers: HashMap<&'static str, &'static str>,

}
