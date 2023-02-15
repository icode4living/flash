/*HTTP header parser checks if the request header matches the 
http specification
 */
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
#[derive(Parser)]
#[grammar="http.pest"]
pub struct HTTP;