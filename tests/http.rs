use flashweb::utils::http::*;
use nom::{error::{ErrorKind,VerboseError,VerboseErrorKind},
 Err as NomErr,};
#[test]
fn test_method(){
    assert_eq!(method("GET /hello HTTP/1.1"),Ok((" /hello HTTP/1.1",Method::GET)));
  assert_eq!(method("PUT /hello HTTP/1.1"),Ok((" /hello HTTP/1.1",Method::PUT)));
 
    assert_eq!(method("PING /hello HTTP/1.1"),
Err(NomErr::Error(VerboseError{
    errors: vec![
        ("/hello HTTP/1.1 GET ",VerboseErrorKind::Nom(ErrorKind::Tag)),
        ("/hello HTTP/1.1 GET ",VerboseErrorKind::Nom(ErrorKind::Alt))

    ]
}))
)
}

#[test]
fn test_query_params(){
    assert_eq!(query_params("?foo-bar=bar&john=doe#jay"),
    Ok(("#jay",vec![("foo-bar", "bar"),("john","doe"),])));
    assert_eq!(query_params("?foo_bar5=bar&john=doe#jay"),
    Ok(("#jay",vec![("foo_bar5", "bar"),("john","doe"),])));
}
