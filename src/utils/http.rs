//HTTP methods
#[derive(Debug, PartialEq, Eq)]
pub enum Method{
GET,
POST,
PUT,
DELETE,
HEAD,
CONNECT,
OPTIONS,
TRACE,
}
//Header e.g Content-Type: text/html
#[derive(Debug)]
pub enum Header{
Token(&'static str),
Value (&'static str),
}
//URI parameters e.g ?key1=value&key2=value
#[derive(Debug)]
pub enum Params{
    Key(&'static str),
    Value(&'static str)
}
#[derive(Debug)]
pub struct URI{
pub path: &'static str,
pub param:Option<Vec<Params>>,
}
#[derive(Debug)]
/*Request line according to RFC 9110 standard
method path version

*/
pub struct RequestLine{
    pub method: Method,
    pub url : URI,
    pub version: Option< &'static str>
}
//HTTP Request header representation according to RFC 9110 standard
#[derive(Debug)]
pub struct Request{
    pub request_line: RequestLine,
    pub header: Option<Vec<Header>>,
    pub body: Option<& 'static str>
}