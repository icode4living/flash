use flashweb::utils::status::*;

#[test]
pub fn test_continue_status(){
    let continue_status = Status{code:"100".to_string(),text:"Continue".to_string()};
    assert_eq!(continue_status.code,Informational::Continue.set().code);
    assert_eq!(continue_status.text, Informational::Continue.set().text);
}
#[test]
pub fn test_success_status(){
    let success_ok = Status{code:"200".to_string(),text:"OK".to_string()};
    assert_eq!(success_ok.code,Success::Ok.set().code);
    assert_eq!(success_ok.text, Success::Ok.set().text);
}