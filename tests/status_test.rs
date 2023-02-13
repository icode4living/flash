use flashweb::utils::status::*;

#[test]
pub fn test_continue_status(){
    let continue_status = Status{code:&"100",text:&"Continue"};
    assert!(continue_status.code == Informational::Continue.set().code);
    assert!(continue_status.text == Informational::Continue.set().text);
}
#[test]
pub fn test_success_status(){
    let success_ok = Status{code:&"200",text:&"OK"};
    assert!(success_ok.code == Success::Ok.set().code);
    assert!(success_ok.text == Success::Ok.set().text);
}
#[test]
pub fn test_redirect_response(){
    let redirect_response = Status{code:&"308",text:&"Permanent Redirect"};
    assert!(redirect_response.code == Redirection::PermanentRedirect.set().code);
    assert!(redirect_response.text == Redirection::PermanentRedirect.set().text);
}

#[test]
pub fn test_client_error(){
    let client_error_response = Status{code:&"400",text:&"Bad Request"};
    assert!(client_error_response.code == ClientError::BadRequest.set().code);
    assert!(client_error_response.text == ClientError::BadRequest.set().text);
}