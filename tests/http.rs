use flashweb::utils::http::*;
use std::fs;

#[test]
fn test_file(){
    let file = fs::read_to_string("sample/test_file.http")
    .expect("could not parse file");
    println!("{:?}",parse_http(file));
}