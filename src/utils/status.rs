
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
//Client Error response
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
    //RequestUriTooLong,
    UnsupportedMediaType,
    //RequestedRangeNotSatisfiable,
    ExpectationFailed,
   MisdirectedRequest,
    UnprocessableEntity,
   // Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
   // ConnectionClosedWithoutResponse,
  //  ClientClosedRequest,
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
//Status Stuct
#[derive(Debug, PartialEq)]

pub struct Status<'a>{
pub code: &'a str,
pub text: & 'a str,
}
impl<'a> PartialEq<&str> for Status<'static>

{
    fn eq(&self, other: &&str) -> bool {
        self == other
    }
}

pub trait Format<'a>
where 'a: 'static
{
    
    fn set(self)->Status<'static>;
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
 /// assert!(&continue_status.code== &"100");
 /// assert!(&continue_status.text==&"Continue");
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
 /// assert!(&success_status.code==&"200");
 /// assert!(&success_status.text==&"OK");
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
 /// assert!(&redirect_response.code==&"308");
 /// assert!(&redirect_response.text == &"Permanent Redirect");
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
      /// # Generate status code and text for **[HTTP Server response](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_response)**
    /// The status enums implements the trait set to generate http standard status code and text.
    ///
    /// ##Example
  /// ```
///  use flashweb::utils::status::*;
///  
 /// let server_response= ServerError::InternalServerError.set();
 /// assert!(&server_response.code==&"500");
 /// assert!(&server_response.text == &"Internal Server Error");
 /// ``` 
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

impl Format<'static> for ClientError{
       /// # Generate status code and text for **[Client Error response](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#client_error_response)**
    /// The status enums implements the trait set to generate http standard status code and text.
    ///
    /// ##Example
  /// ```
///  use flashweb::utils::status::*;
///  
 /// let client_response= ClientError::BadRequest.set();
 /// assert!(&client_response.code==&"400");
 /// assert!(&client_response.text == &"Bad Request");
 /// ``` 
    fn set(self)->Status<'static> {
        match self {
            Self::BadRequest=> Status { code: &"400", text: &"Bad Request"},
            Self::Unauthorized => Status { code: &"401", text: &"Unauthorized" },
            Self::Forbidden => Status { code: &"403", text: &"Forbidden" },
            Self::NotFound => Status { code: &"404", text: &"Not Found" },
            Self::MethodNotAllowed => Status { code: &"405", text: &"Method Not Allowed" },
            Self::ProxyAuthenticationRequired => Status { code: &"407", text: &"Proxy Authentication Required" },
            Self::RequestTimeout => Status { code: &"408", text: &"Request Timeout" },
            Self::Conflict => Status { code: &"409", text: &"Conflict" },
            Self::Gone => Status { code: &"410", text: &"Gone" },
            Self::LengthRequired => Status { code: &"411", text: &"Length Required" },
            Self::PreconditionFailed => Status { code: &"412", text: &"Precondition Failed" },
            Self::PayloadTooLarge => Status { code: &"413", text: &"Payload Too Large" },
            Self::UnsupportedMediaType => Status { code: &"415", text: &"Unsupported Media Type" },
            Self::ExpectationFailed => Status { code: &"417", text: &"Expectation Failed" },
            Self::MisdirectedRequest => Status { code: &"421", text: &"Misdirected Request" },
            Self::UpgradeRequired => Status { code: &"200", text: &"Upgrade Required" },
            Self::TooManyRequests => Status { code: &"429", text: &"Too Many Request" },
            Self::RequestHeaderFieldsTooLarge =>Status { code: &"431", text: &"Request HeaderFields Too Large" },
            Self::PaymentRequired => Status { code: &"402", text: &"Payment Required" },
            Self::NotAcceptable => Status { code: &"406", text: &"Not Acceptable" },
            Self::UnprocessableEntity => Status { code: &"422", text: &"Unprocessable Entity"},
            Self::FailedDependency => Status { code: &"424", text: &"FailedDependency" },
            Self::PreconditionRequired => Status { code: &"428", text: &"Precondition Required" },

            

        }
    }
}
