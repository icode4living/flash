

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
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NetworkAuthenticationRequired,
    NetworkConnectTimeoutError,
}
//http status code formatter
#[derive(Debug)]
pub struct Status{
pub code: String,
pub text: String,
}
pub trait Format{
    fn set(self)->Status;
}
//Generate status code for Informational response
impl Format for Informational{
    /// # Generate status code and text for **[HTTP Informational responses](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#information_responses)**
    /// The status enums implements the trait set to generate http standard status code and text.
    ///
    /// ##Example
  /// ```
///  use flashweb::utils::status::*;
/// 
///  
 /// let continue_status = Informational::Continue.set();
 /// assert_eq!(continue_status.code, "100");
 /// assert_eq!(continue_status.text, "Continue");
 /// ```  
  fn set(self)->Status{
    
    match self{
     Informational::Continue => Status{code:"100".to_string(),
     text:"Continue".to_string()},
     Informational::SwitchingProtocols => Status{code:"101".to_string(),
     text:"Switching Protocols".to_string()},
     Informational::EarlyHints=> Status{code:"102".to_string(),
     text:"Early Hints".to_string()},  
    }
 }
}
//HTTP Success response code
impl Format for Success {
     /// # Generate status code and text for **[HTTP Successessful responses](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#successful_responses)**
    /// The status enums implements the trait set to generate http standard status code and text.
    ///
    /// ##Example
  /// ```
///  use flashweb::utils::status::*;
///  
 /// let success_status = Success::Ok.set();
 /// assert_eq!(success_status .code, "200");
 /// assert_eq!(success_status .text, "OK");
 /// ``` 
    fn set(self)->Status {
        match self {
            Self::Accepted => Status { code: "202".to_string(), text: "Accepted".to_string()},
            Self::AlreadyReported =>Status { code: "208".to_string(), text:"Already Reported".to_string()},
            Self::Created => Status { code: "201".to_string(), text: "Created".to_string()},
            Self::ImUsed => Status { code: "226".to_string(), text: "IM used".to_string() },
            Self::MultiStatus => Status { code: "207".to_string(), text: "Multi-Status".to_string() },
            Self::NoContent => Status { code: "204".to_string(), text: "No Content" .to_string()},
            Self::NonAuthoritativeInformation => Status { code: "203".to_string(), 
            text: "Non-Authoritative Information".to_string()},
            Self::Ok => Status { code: "200".to_string(), text: "OK".to_string() },
            Self::PartialContent => Status {code: "206".to_string(), text:"Partial Content".to_string()},
            Self::ResetContent => Status { code: "205".to_string(), text: "Reset Content".to_string() },
        }
    }
}