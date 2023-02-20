use flashweb::utils::http_parser::*;
use std::fs;

#[test]
pub fn test_http(){
    let file = fs::read_to_string("sample/test_file.http")
    .expect("file could not be read");
    let f = store_http(file);
    println!("{}",f["request"]["cookie"])

}

