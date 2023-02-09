/**HTTP status code */
#[derive(Debug)]
pub enum Informational{
    Continue,
    SwitchingProtocols,
    Processing,
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
code: String,
text: String,
}
///This Trait returns the http code and text
pub trait format{
    pub fn format(self)->Status;
}
impl format for Informational{
 fn format(self)->Status{
    match self{
        
    }
 }
}