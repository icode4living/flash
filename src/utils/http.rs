use std::{collections::HashMap};
use nom::{ InputTakeAtPosition, sequence::{tuple, terminated}, multi::*, combinator::*,
IResult, error::{VerboseError, context, ErrorKind}, bytes::streaming::*,
branch::alt, AsChar
};
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

impl From<&str> for Method{
    
    fn from(value: &str) -> Self{
        match value.to_uppercase().as_str(){
        "GET"=>Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
       "DELETE" => Method::DELETE,
        "HEAD" => Method::HEAD,
        "CONNECT" => Method::CONNECT,
        "OPTIONS" => Method::OPTIONS,
        "TRACE" => Method::TRACE,

        _ => unimplemented!("Scheme is not supported")
        }
    }
}
//Header e.g Content-Type: text/html 
pub type Header = HashMap<&'static str, & 'static str>;
//URI parameters e.g ?key1=value&key2=value
pub type Param<'a> = (&'a str, &'a str); 
pub type Params<'a> = Vec<Param<'a>>;
pub type Path<'a> = Vec<&'a str>;
//pub type URI <'a> = (Path<'a>, Option<Vec<Params<'a>>>);
#[derive(Debug,PartialEq, Eq)]
pub struct URI<'a>{
   pub path: Path<'a>,
   pub query: Option<Params<'a>>
}
#[derive(Debug)]
/*Request line according to RFC 9110 standard
method path version

*/
pub struct RequestLine<'a>{
    pub method: Method,
    pub url : URI<'a>,
    pub version: Option< &'static str>
}
//HTTP Request header representation according to RFC 9110 standard
#[derive(Debug)]
pub struct Request<'a>{
    pub request_line: RequestLine<'a>,
    pub header: Option<Vec<Header>>,
    pub body: Option<& 'static str>
}
//Custom error for nom
type Res<T, U> = IResult<T, U, VerboseError<T>>;

//parse method
pub fn method(input: &str)-> Res<&str, Method>{
context("method", 
alt((tag_no_case("GET"),tag_no_case("POST"),tag_no_case("DELETE"),tag_no_case("PUT"),
tag_no_case("HEAD"),tag_no_case("CONNECT"),tag_no_case("OPTIONS"),tag_no_case("TRACE")))
)(input).map(|(next_iput,res)|(next_iput,res.into()))

}


//uri converter
fn url_format<T>(i:T)->Res<T,T>
    where
        T: InputTakeAtPosition,
        <T as InputTakeAtPosition>::Item: AsChar,
        {
            i.split_at_position1_complete(
                |item|{
                    let char_item = item.as_char();
                    !(char_item == '-') && !char_item.is_alphanumeric()&& !(char_item == '/')&&  
                    !(char_item == '_')
                },
                ErrorKind::AlphaNumeric
            )
        }
//query params ?name=Doe&age=5
pub fn query_params(input:&str)->Res<&str, Params>{
    
    context("query params", 
    tuple((
        tag("?"),
        url_format,
        tag("="),
        url_format,
        many0(tuple((
            tag("&"),
            url_format,
            tag("="),
            url_format,
        ))),
    )),

)(input)
.map(|(next_input,res)|{
    let mut qps = Vec::new();
   // let mut qkey = HashMap::new();
   //qps.push(qkey.insert(res.1, res.3));
   qps.push((res.1, res.3));

    for qp in res.4{
      //qps.push(qkey.insert(qp.1, qp.3));
      qps.push((qp.1, qp.3));

    }
    (next_input,qps)

})



}

//parse path 
/**TODO test path */
pub fn path(input: &str)->Res<&str, Path>{
    context("path", 
    
tuple((tag("/"),
many0(terminated(url_format, tag("/"))),
opt(url_format)
))
)(input)
    .map(|(next_input, res)|{
        let mut paths = Vec::new();
        if let Some(last) = res.2{
            paths.push(last);
        }
        (next_input,paths)   
    })
    
}

//parse URI
/** example/hello?foo=bar&jhone=doe */
pub fn uri(input: &str)->Res<&str, URI>{
   context(
    "uri",
tuple((path, opt(query_params))),
   )(input)
   .map(|(next_input,res)|{
    let (p, query_p) = res;
    (next_input,
    URI{
        path:p,
        query:query_p,
    })
   }
)
    
}