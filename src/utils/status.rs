use syn::token::For;

///Todo
/// [-] implement PartialEq for static str
/// [-] Test 
/// [-] complete client request

//HTTP status code 
#[derive(Debug,PartialEq)]
pub enum Informational{


    Continue,
    SwitchingProtocols,
    EarlyHints,

}
#[derive(Debug)]
pub enum Success{
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed,
}
#[derive(Debug)]
pub enum Redirection{
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
}
#[derive(Debug)]
pub enum ClientError{
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    RequestUriTooLong,
    UnsupportedMediaType,
    RequestedRangeNotSatisfiable,
    ExpectationFailed,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    ConnectionClosedWithoutResponse,
    ClientClosedRequest,
}
pub enum ServerError{
  InternalServerError,
   // NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NetworkAuthenticationRequired,
   // NetworkConnectTimeoutError,
}
//http status code formatter
#[derive(Debug)]
pub struct Status<'a>{
pub code: &'a str,
pub text: & 'a str,
}
pub trait Format<'a>
where 'a: 'static
{
    
    fn set(self)->Status<'a>;
}
//Generate status code for Informational response
impl Format<'static> for Informational
{
    /// # Generate status code and text for **[HTTP Informational responses](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#information_responses)**
    /// The status enums implements the trait set to generate http standard status code and text.
    ///
    /// ##Example
  /// ```
///  use flashweb::utils::status::*;
/// 
///  
 /// let continue_status = Informational::Continue.set();
 /// assert_eq!(continue_status.code, &"100");
 /// assert_eq!(continue_status.text, &"Continue");
 /// ```  
  fn set(self)->Status<'static>{
    
    match self{
     Informational::Continue => Status{code:&"100",
     text: &"Continue"},
     Informational::SwitchingProtocols => Status{code:&"101",
     text:&"Switching Protocols"},
     Informational::EarlyHints=> Status{code:&"102",
     text:&"Early Hints"},  
    }
 }
}
//HTTP Success response code
impl Format <'static> for Success {
     /// # Generate status code and text for **[HTTP Successessful responses](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#successful_responses)**
    /// The status enums implements the trait set to generate http standard status code and text.
    ///
    /// ##Example
  /// ```
///  use flashweb::utils::status::*;
///  
 /// let success_status = Success::Ok.set();
 /// assert_eq!(success_status.code, &"200");
 /// assert_eq!(success_status.text, &"OK");
 /// ``` 
    fn set(self)->Status<'static> {
        match self {
            Self::Accepted => Status { code: &"202", text: &"Accepted"},
            Self::AlreadyReported =>Status { code: &"208", text:&"Already Reported"},
            Self::Created => Status { code: &"201", text: &"Created"},
            Self::ImUsed => Status { code: &"226", text: &"IM used" },
            Self::MultiStatus => Status { code: &"207", text: &"Multi-Status"},
            Self::NoContent => Status { code: &"204", text: &"No Content"},
            Self::NonAuthoritativeInformation => Status { code: &"203", 
            text: &"Non-Authoritative Information"},
            Self::Ok => Status { code: &"200", text: &"OK"},
            Self::PartialContent => Status {code: &"206", text:&"Partial Content"},
            Self::ResetContent => Status { code: &"205", text: &"Reset Content"},
        }
    }
}

//HTTP Redirection Responses
impl Format<'static> for Redirection{
      /// # Generate status code and text for **[HTTP Successessful responses](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#redirection_messages)**
    /// The status enums implements the trait set to generate http standard status code and text.
    ///
    /// ##Example
  /// ```
///  use flashweb::utils::status::*;
///  
 /// let redirect_response= Redirection::PermanentRedirect.set();
 /// assert_eq!(redirect_response.code, &"308");
 /// assert_eq!(redirect_response.text, &"Permanent Redirect");
 /// ``` 
    fn set(self)->Status<'static> {
        match self {
          Self::MultipleChoices=>Status { code: &"300", text: &"Multiple Choices"},
          Self::MovedPermanently=>Status { code: &"301", text: &"Moved Permanently"},
          Self::Found => Status { code: &"302", text: &"Found"},
          Self::SeeOther => Status { code: &"303", text: &"See Other"},
          Self::NotModified => Status { code: &"304", text: &"Not Modified"},
          Self::UseProxy => Status { code: &"305", text: &"Use Proxy"},
          Self::TemporaryRedirect => Status { code: &"307", text: &"Temporary Redirect"},
          Self::PermanentRedirect => Status { code: &"308", text: &"Permanent Redirect"},

        }
    }

}
//HTTP Server Error Responses
impl Format<'static> for ServerError{
    fn set(self)->Status<'static> {
        match self{
            Self::InternalServerError=>Status { code: &"500", text: &"Internal Server Error"},
            Self::BadGateway=>Status { code: &"502", text: &"Bad Gateway"},
            Self::ServiceUnavailable => Status { code: &"503", text: &"Service Unavailable"},
            Self::GatewayTimeout => Status { code: &"504", text: &"Gateway Timeout"},
            Self::HttpVersionNotSupported => Status { code: &"505", text: &"HTTP Version Not Supported"},
            Self::VariantAlsoNegotiates => Status { code: &"506", text: &"Variant Also Negotiates"},
            Self::InsufficientStorage => Status { code: &"507", text: &"Insufficient Storage"},
            Self::LoopDetected => Status { code: &"508", text: &"Loop Detected"},
            Self::NetworkAuthenticationRequired => Status { code: &"511", text: &"Network Authentication Required"},
        }
    }
}
/*
impl Format for ClientError{
    fn set(self)->Status {
        match self {
            Self::BadRequest=> Status { code: &"400", text: "Bad Request"},
            Self::Unauthorized => Status { code: &"401", text: &"Unauthorized" }
        }
    }
}
*/