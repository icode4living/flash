/**HTTP status code */
#[derive(Debug, PartialEq)]
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
pub struct Status{
pub code: String,
pub text: String,
}
///This Trait returns the http code and text
pub trait Format{
    pub fn get(self)->Status;
}
/** Informational */
impl Format for Informational{
 fn get(self)->Status{
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
/**Success message */

