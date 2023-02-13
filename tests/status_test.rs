use flashweb::utils::status::*;

#[test]
pub fn test_continue_status(){
    let continue_status = Status{code:&"100",text:&"Continue"};
    assert_eq!(continue_status.code,Informational::Continue.set().code);
    assert_eq!(continue_status.text, Informational::Continue.set().text);
}
#[test]
pub fn test_success_status(){
    let success_ok = Status{code:&"200",text:&"OK"};
    assert_eq!(success_ok.code,Success::Ok.set().code);
    assert_eq!(success_ok.text, Success::Ok.set().text);
}
#[test]
pub fn test_redirect_response(){
    let redirect_response = Status{code:&"308",text:&"Permanent Redirect"};
    assert_eq!(redirect_response.code,Redirection::PermanentRedirect.set().code);
    assert_eq!(redirect_response.text, Redirection::PermanentRedirect.set().text);
}