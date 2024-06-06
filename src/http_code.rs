// This file is part of http_codex <https://github.com/nevermille/http_codex>
// Copyright (C) 2024 Camille Nevermind
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

#[derive(Copy, Clone)]
/// The HTTP codes with all their descriptions. Thanks to MDN for the documentation
pub enum HttpCode {
    /// Code 100
    ///
    /// This interim response indicates that the client should continue the request or ignore
    /// the response if the request is already finished.
    Continue,

    /// Code 101
    ///
    /// This code is sent in response to an Upgrade request header from the client and
    /// indicates the protocol the server is switching to.
    SwitchingProtocols,

    /// Code 102 (WebDAV)
    ///
    /// This code indicates that the server has received and is processing the request,
    /// but no response is available yet.
    Processing,

    /// Code 103
    ///
    /// This status code is primarily intended to be used with the Link header, letting the user
    /// agent start preloading resources while the server prepares a response or preconnect to
    /// an origin from which the page will need resources.
    EarlyHints,

    /// Code 200
    ///
    /// The request succeeded. The result meaning of "success" depends on the HTTP method:
    ///
    /// * `GET`: The resource has been fetched and transmitted in the message body.
    /// * `HEAD`: The representation headers are included in the response without any message body.
    /// * `PUT` or `POST`: The resource describing the result of the action is transmitted in the message body.
    /// * `TRACE`: The message body contains the request message as received by the server.
    Ok,

    /// Code 201
    ///
    /// The request succeeded, and a new resource was created as a result. This is typically
    /// the response sent after `POST` requests, or some `PUT` requests.
    Created,

    /// Code 202
    ///
    /// The request has been received but not yet acted upon. It is noncommittal, since there is
    /// no way in HTTP to later send an asynchronous response indicating the outcome of the
    /// request. It is intended for cases where another process or server handles the request,
    /// or for batch processing.
    Accepted,

    /// Code 203
    ///
    /// This response code means the returned metadata is not exactly the same as is available
    /// from the origin server, but is collected from a local or a third-party copy. This
    /// is mostly used for mirrors or backups of another resource. Except for that specific case,
    /// the `200 OK` response is preferred to this status.
    NonAuthoritativeInformation,

    /// Code 204
    ///
    /// There is no content to send for this request, but the headers may be useful. The user
    /// agent may update its cached headers for this resource with the new ones.
    NoContent,

    /// Code 205
    ///
    /// Tells the user agent to reset the document which sent this request.
    ResetContent,

    /// Code 206
    ///
    /// This response code is used when the Range header is sent from the client to request
    /// only part of a resource.
    PartialContent,

    /// Code 207 (WebDAV)
    ///
    /// Conveys information about multiple resources, for situations where multiple status codes
    /// might be appropriate.
    MultiStatus,

    /// Code 208 (WebDAV)
    ///
    /// Used inside a `<dav:propstat>` response element to avoid repeatedly enumerating the
    /// internal members of multiple bindings to the same collection.
    AlreadyReported,

    /// Code 226 (HTTP Delta encoding)
    ///
    /// The server has fulfilled a `GET` request for the resource, and the response is a
    /// representation of the result of one or more instance-manipulations applied to the
    /// current instance.
    ImUsed,

    /// Code 300
    ///
    /// The request has more than one possible response. The user agent or user should choose
    /// one of them. (There is no standardized way of choosing one of the responses, but HTML
    /// links to the possibilities are recommended so the user can pick.)
    MultipleChoices,

    /// Code 301
    ///
    /// The URL of the requested resource has been changed permanently. The new URL
    /// is given in the response.
    MovedPermanently,

    /// Code 302
    ///
    /// This response code means that the URI of requested resource has been changed temporarily.
    /// Further changes in the URI might be made in the future. Therefore, this same URI should
    /// be used by the client in future requests.
    Found,

    /// Code 303
    ///
    /// The server sent this response to direct the client to get the requested resource at
    /// another URI with a GET request.
    SeeOther,

    /// Code 304
    ///
    /// This is used for caching purposes. It tells the client that the response has not been
    /// modified, so the client can continue to use the same cached version of the response.
    NotModified,

    /// Code 307
    ///
    /// The server sends this response to direct the client to get the requested resource
    /// at another URI with the same method that was used in the prior request. This has the
    /// same semantics as the `302 Found` HTTP response code, with the exception that the user
    /// agent must not change the HTTP method used: if a `POST` was used in the first request,
    /// a `POST` must be used in the second request.
    TemporaryRedirect,

    /// Code 308
    ///
    /// This means that the resource is now permanently located at another URI, specified by the
    /// `Location:` HTTP Response header. This has the same semantics as the `301 Moved Permanently`
    /// HTTP response code, with the exception that the user agent must not change the HTTP method
    /// used: if a `POST` was used in the first request, a `POST` must be used in the second request.
    PermanentRedirect,

    /// Code 400
    ///
    /// The server cannot or will not process the request due to something that is perceived
    /// to be a client error (e.g., malformed request syntax, invalid request message framing,
    /// or deceptive request routing).
    BadRequest,

    /// Code 401
    ///
    /// Although the HTTP standard specifies "unauthorized", semantically this response
    /// means "unauthenticated". That is, the client must authenticate itself to get
    /// the requested response.
    Unauthorized,

    /// Code 402 (Experimental)
    ///
    /// This response code is reserved for future use. The initial aim for creating this code was
    /// using it for digital payment systems, however this status code is used very rarely
    /// and no standard convention exists.
    PaymentRequired,

    /// Code 403
    ///
    /// The client does not have access rights to the content; that is, it is unauthorized, so the
    /// server is refusing to give the requested resource. Unlike `401 Unauthorized`, the client's
    /// identity is known to the server.
    Forbidden,

    /// Code 404
    ///
    /// The server cannot find the requested resource. In the browser, this means the URL is not
    /// recognized. In an API, this can also mean that the endpoint is valid but the resource
    /// itself does not exist. Servers may also send this response instead of `403 Forbidden` to
    /// hide the existence of a resource from an unauthorized client. This response code is
    /// probably the most well known due to its frequent occurrence on the web.
    NotFound,

    /// Code 405
    ///
    /// The request method is known by the server but is not supported by the target resource.
    /// For example, an API may not allow calling `DELETE` to remove a resource.
    MethodNotAllowed,

    /// Code 406
    ///
    /// This response is sent when the web server, after performing server-driven content
    /// negotiation, doesn't find any content that conforms to the criteria given by the user agent.
    NotAcceptable,

    /// Code 407
    ///
    /// This is similar to `401 Unauthorized` but authentication is needed to be done by a proxy.
    ProxyAuthentificationRequired,

    /// Code 408
    ///
    /// This response is sent on an idle connection by some servers, even without any previous
    /// request by the client. It means that the server would like to shut down this unused
    /// connection. This response is used much more since some browsers, like Chrome,
    /// Firefox 27+, or IE9, use HTTP pre-connection mechanisms to speed up surfing. Also
    /// note that some servers merely shut down the connection without sending this message.
    RequestTimeout,

    /// Code 409
    ///
    /// This response is sent when a request conflicts with the current state of the server.
    Conflict,

    /// Code 410
    ///
    /// This response is sent when the requested content has been permanently deleted from
    /// server, with no forwarding address. Clients are expected to remove their caches
    /// and links to the resource. The HTTP specification intends this status code to be used
    /// for "limited-time, promotional services". APIs should not feel compelled to indicate
    /// resources that have been deleted with this status code.
    Gone,

    /// Code 411
    ///
    /// Server rejected the request because the Content-Length header field is not defined and the
    /// server requires it.
    LengthRequired,

    /// Code 412
    ///
    /// The client has indicated preconditions in its headers which the server does not meet.
    PreconditionFailed,

    /// Code 413
    ///
    /// Request entity is larger than limits defined by server. The server might close the
    /// connection or return an Retry-After header field.
    PayloadTooLarge,

    /// Code 414
    ///
    /// The URI requested by the client is longer than the server is willing to interpret.
    UriTooLong,

    /// Code 415
    ///
    /// The media format of the requested data is not supported by the server, so the server
    /// is rejecting the request.
    UnsupportedMediaType,

    /// Code 416
    ///
    /// The range specified by the `Range` header field in the request cannot be fulfilled.
    /// It's possible that the range is outside the size of the target URI's data.
    RangeNotSatisfiable,

    /// Code 417
    ///
    /// This response code means the expectation indicated by the Expect request header field
    /// cannot be met by the server.
    ExpectationFailed,

    /// Code 418
    ///
    /// The server refuses the attempt to brew coffee with a teapot.
    ImATeapot,

    /// Code 421
    ///
    /// The request was directed at a server that is not able to produce a response. This can
    /// be sent by a server that is not configured to produce responses for the combination
    /// of scheme and authority that are included in the request URI.
    MisdirectedRequest,

    /// Code 422 (WebDAV)
    ///
    /// The request was well-formed but was unable to be followed due to semantic errors.
    UnprocessableContent,

    /// Code 423 (WebDAV)
    ///
    /// The resource that is being accessed is locked.
    Locked,

    /// Code 424 (WebDAV)
    ///
    /// The request failed due to failure of a previous request.
    FailedDependency,

    /// Code 425 (Experimental)
    ///
    /// Indicates that the server is unwilling to risk processing a request that might be replayed.
    TooEarly,

    /// Code 426
    ///
    /// The server refuses to perform the request using the current protocol but might be
    /// willing to do so after the client upgrades to a different protocol. The server sends
    /// an `Upgrade` header in a 426 response to indicate the required protocol(s).
    UpgradeRequired,

    /// Code 428
    ///
    /// The origin server requires the request to be conditional. This response is intended
    /// to prevent the 'lost update' problem, where a client GETs a resource's state, modifies
    /// it and PUTs it back to the server, when meanwhile a third party has modified the state
    /// on the server, leading to a conflict.
    PreconditionRequired,

    /// Code 429
    ///
    /// The user has sent too many requests in a given amount of time ("rate limiting").
    TooManyRequests,

    /// Code 431
    ///
    /// The server is unwilling to process the request because its header fields are too
    /// large. The request may be resubmitted after reducing the size of the request header fields.
    RequestHeaderFieldsTooLarge,

    /// Code 451
    ///
    /// The user agent requested a resource that cannot legally be provided, such as a web page
    /// censored by a government.
    UnavailableForLegalReasons,

    /// Code 500
    ///
    /// The server has encountered a situation it does not know how to handle.
    InternalServerError,

    /// Code 501
    ///
    /// The request method is not supported by the server and cannot be handled. The only
    /// methods that servers are required to support (and therefore that must not return
    /// this code) are `GET` and `HEAD`.
    NotImplemented,

    /// Code 502
    ///
    /// This error response means that the server, while working as a gateway to get a response
    /// needed to handle the request, got an invalid response.
    BadGateway,

    /// Code 503
    ///
    /// The server is not ready to handle the request. Common causes are a server that is
    /// down for maintenance or that is overloaded. Note that together with this response,
    /// a user-friendly page explaining the problem should be sent. This response should be
    /// used for temporary conditions and the `Retry-After` HTTP header should, if possible,
    /// contain the estimated time before the recovery of the service. The webmaster must
    /// also take care about the caching-related headers that are sent along with this response,
    /// as these temporary condition responses should usually not be cached.
    ServiceUnavailable,

    /// Code 504
    ///
    /// This error response is given when the server is acting as a gateway and cannot get
    /// a response in time.
    GatewayTimeout,

    /// Code 505
    ///
    /// The HTTP version used in the request is not supported by the server.
    HttpVersionNotSupported,

    /// Code 506
    ///
    /// The server has an internal configuration error: the chosen variant resource is
    /// configured to engage in transparent content negotiation itself, and is therefore
    /// not a proper end point in the negotiation process.
    VariantAlsoNegotiates,

    /// Code 507 (WebDAV)
    ///
    /// The method could not be performed on the resource because the server is unable to
    /// store the representation needed to successfully complete the request.
    InsufficientStorage,

    /// Code 508 (WebDAV)
    ///
    /// The server detected an infinite loop while processing the request.
    LoopDetected,

    /// Code 510
    ///
    /// Further extensions to the request are required for the server to fulfill it.
    NotExtended,

    /// Code 511
    ///
    /// Indicates that the client needs to authenticate to gain network access.
    NetworkAuthetificationRequired,

    /// No code were given
    None,

    /// A code was given but is unknown to the library
    Unknown(u32),
}

#[derive(Copy, Clone)]
/// HTTP code classes so comparing code's hundreds is not necessary
pub enum HttpCodeClass {
    /// Code 1xx
    Informational,

    /// Code 2xx
    Successful,

    /// Code 3xx
    Redirection,

    /// Code 4xx
    ClientError,

    /// Code 5xx
    ServerError,

    /// No code received
    None,

    /// Unknown code
    Unknown,
}

impl From<u32> for HttpCode {
    fn from(value: u32) -> Self {
        match value {
            100 => HttpCode::Continue,
            101 => HttpCode::SwitchingProtocols,
            102 => HttpCode::Processing,
            103 => HttpCode::EarlyHints,
            200 => HttpCode::Ok,
            201 => HttpCode::Created,
            202 => HttpCode::Accepted,
            203 => HttpCode::NonAuthoritativeInformation,
            204 => HttpCode::NoContent,
            205 => HttpCode::ResetContent,
            206 => HttpCode::PartialContent,
            207 => HttpCode::MultiStatus,
            208 => HttpCode::AlreadyReported,
            226 => HttpCode::ImUsed,
            300 => HttpCode::MultipleChoices,
            301 => HttpCode::MovedPermanently,
            302 => HttpCode::Found,
            303 => HttpCode::SeeOther,
            304 => HttpCode::NotModified,
            307 => HttpCode::TemporaryRedirect,
            308 => HttpCode::PermanentRedirect,
            400 => HttpCode::BadRequest,
            401 => HttpCode::Unauthorized,
            402 => HttpCode::PaymentRequired,
            403 => HttpCode::Forbidden,
            404 => HttpCode::NotFound,
            405 => HttpCode::MethodNotAllowed,
            406 => HttpCode::NotAcceptable,
            407 => HttpCode::ProxyAuthentificationRequired,
            408 => HttpCode::RequestTimeout,
            409 => HttpCode::Conflict,
            410 => HttpCode::Gone,
            411 => HttpCode::LengthRequired,
            412 => HttpCode::PreconditionFailed,
            413 => HttpCode::PayloadTooLarge,
            414 => HttpCode::UriTooLong,
            415 => HttpCode::UnsupportedMediaType,
            416 => HttpCode::RangeNotSatisfiable,
            417 => HttpCode::ExpectationFailed,
            418 => HttpCode::ImATeapot,
            421 => HttpCode::MisdirectedRequest,
            422 => HttpCode::UnprocessableContent,
            423 => HttpCode::Locked,
            424 => HttpCode::FailedDependency,
            425 => HttpCode::TooEarly,
            426 => HttpCode::UpgradeRequired,
            428 => HttpCode::PreconditionRequired,
            429 => HttpCode::TooManyRequests,
            431 => HttpCode::RequestHeaderFieldsTooLarge,
            451 => HttpCode::UnavailableForLegalReasons,
            500 => HttpCode::InternalServerError,
            501 => HttpCode::NotImplemented,
            502 => HttpCode::BadGateway,
            503 => HttpCode::ServiceUnavailable,
            504 => HttpCode::GatewayTimeout,
            505 => HttpCode::HttpVersionNotSupported,
            506 => HttpCode::VariantAlsoNegotiates,
            507 => HttpCode::InsufficientStorage,
            508 => HttpCode::LoopDetected,
            510 => HttpCode::NotExtended,
            511 => HttpCode::NetworkAuthetificationRequired,
            v => HttpCode::Unknown(v),
        }
    }
}

impl Default for HttpCode {
    fn default() -> Self {
        Self::None
    }
}

impl From<u128> for HttpCode {
    fn from(value: u128) -> Self {
        (value as u32).into()
    }
}

impl From<u64> for HttpCode {
    fn from(value: u64) -> Self {
        (value as u32).into()
    }
}

impl From<u16> for HttpCode {
    fn from(value: u16) -> Self {
        (value as u32).into()
    }
}

impl From<usize> for HttpCode {
    fn from(value: usize) -> Self {
        (value as u32).into()
    }
}

impl From<i128> for HttpCode {
    fn from(value: i128) -> Self {
        (value as u32).into()
    }
}

impl From<i64> for HttpCode {
    fn from(value: i64) -> Self {
        (value as u32).into()
    }
}

impl From<i32> for HttpCode {
    fn from(value: i32) -> Self {
        (value as u32).into()
    }
}

impl From<i16> for HttpCode {
    fn from(value: i16) -> Self {
        (value as u32).into()
    }
}

impl From<isize> for HttpCode {
    fn from(value: isize) -> Self {
        (value as u32).into()
    }
}

impl From<Option<u128>> for HttpCode {
    fn from(value: Option<u128>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<Option<u64>> for HttpCode {
    fn from(value: Option<u64>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<Option<u32>> for HttpCode {
    fn from(value: Option<u32>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<Option<u16>> for HttpCode {
    fn from(value: Option<u16>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<Option<usize>> for HttpCode {
    fn from(value: Option<usize>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<Option<i128>> for HttpCode {
    fn from(value: Option<i128>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<Option<i64>> for HttpCode {
    fn from(value: Option<i64>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<Option<i32>> for HttpCode {
    fn from(value: Option<i32>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<Option<i16>> for HttpCode {
    fn from(value: Option<i16>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<Option<isize>> for HttpCode {
    fn from(value: Option<isize>) -> Self {
        match value {
            None => HttpCode::None,
            Some(v) => HttpCode::from(v),
        }
    }
}

impl From<HttpCode> for u32 {
    fn from(value: HttpCode) -> Self {
        match value {
            HttpCode::Continue => 100,
            HttpCode::SwitchingProtocols => 101,
            HttpCode::Processing => 102,
            HttpCode::EarlyHints => 103,
            HttpCode::Ok => 200,
            HttpCode::Created => 201,
            HttpCode::Accepted => 202,
            HttpCode::NonAuthoritativeInformation => 203,
            HttpCode::NoContent => 204,
            HttpCode::ResetContent => 205,
            HttpCode::PartialContent => 206,
            HttpCode::MultiStatus => 207,
            HttpCode::AlreadyReported => 208,
            HttpCode::ImUsed => 226,
            HttpCode::MultipleChoices => 300,
            HttpCode::MovedPermanently => 301,
            HttpCode::Found => 302,
            HttpCode::SeeOther => 303,
            HttpCode::NotModified => 304,
            HttpCode::TemporaryRedirect => 307,
            HttpCode::PermanentRedirect => 308,
            HttpCode::BadRequest => 400,
            HttpCode::Unauthorized => 401,
            HttpCode::PaymentRequired => 402,
            HttpCode::Forbidden => 403,
            HttpCode::NotFound => 404,
            HttpCode::MethodNotAllowed => 405,
            HttpCode::NotAcceptable => 406,
            HttpCode::ProxyAuthentificationRequired => 407,
            HttpCode::RequestTimeout => 408,
            HttpCode::Conflict => 409,
            HttpCode::Gone => 410,
            HttpCode::LengthRequired => 411,
            HttpCode::PreconditionFailed => 412,
            HttpCode::PayloadTooLarge => 413,
            HttpCode::UriTooLong => 414,
            HttpCode::UnsupportedMediaType => 415,
            HttpCode::RangeNotSatisfiable => 416,
            HttpCode::ExpectationFailed => 417,
            HttpCode::ImATeapot => 418,
            HttpCode::MisdirectedRequest => 421,
            HttpCode::UnprocessableContent => 422,
            HttpCode::Locked => 423,
            HttpCode::FailedDependency => 424,
            HttpCode::TooEarly => 425,
            HttpCode::UpgradeRequired => 426,
            HttpCode::PreconditionRequired => 428,
            HttpCode::TooManyRequests => 429,
            HttpCode::RequestHeaderFieldsTooLarge => 431,
            HttpCode::UnavailableForLegalReasons => 451,
            HttpCode::InternalServerError => 500,
            HttpCode::NotImplemented => 501,
            HttpCode::BadGateway => 502,
            HttpCode::ServiceUnavailable => 503,
            HttpCode::GatewayTimeout => 504,
            HttpCode::HttpVersionNotSupported => 505,
            HttpCode::VariantAlsoNegotiates => 506,
            HttpCode::InsufficientStorage => 507,
            HttpCode::LoopDetected => 508,
            HttpCode::NotExtended => 510,
            HttpCode::NetworkAuthetificationRequired => 511,
            HttpCode::Unknown(v) => v,
            HttpCode::None => 0,
        }
    }
}

impl From<HttpCode> for Option<u32> {
    fn from(value: HttpCode) -> Self {
        match value {
            HttpCode::None => None,
            _ => Some(value.into()),
        }
    }
}

impl From<HttpCode> for HttpCodeClass {
    fn from(value: HttpCode) -> Self {
        match value {
            HttpCode::Continue
            | HttpCode::SwitchingProtocols
            | HttpCode::Processing
            | HttpCode::EarlyHints => HttpCodeClass::Informational,

            HttpCode::Ok
            | HttpCode::Created
            | HttpCode::Accepted
            | HttpCode::NonAuthoritativeInformation
            | HttpCode::NoContent
            | HttpCode::ResetContent
            | HttpCode::PartialContent
            | HttpCode::MultiStatus
            | HttpCode::AlreadyReported
            | HttpCode::ImUsed => HttpCodeClass::Successful,

            HttpCode::MultipleChoices
            | HttpCode::MovedPermanently
            | HttpCode::Found
            | HttpCode::SeeOther
            | HttpCode::NotModified
            | HttpCode::TemporaryRedirect
            | HttpCode::PermanentRedirect => HttpCodeClass::Redirection,

            HttpCode::BadRequest
            | HttpCode::Unauthorized
            | HttpCode::PaymentRequired
            | HttpCode::Forbidden
            | HttpCode::NotFound
            | HttpCode::MethodNotAllowed
            | HttpCode::NotAcceptable
            | HttpCode::ProxyAuthentificationRequired
            | HttpCode::RequestTimeout
            | HttpCode::Conflict
            | HttpCode::Gone
            | HttpCode::LengthRequired
            | HttpCode::PreconditionFailed
            | HttpCode::PayloadTooLarge
            | HttpCode::UriTooLong
            | HttpCode::UnsupportedMediaType
            | HttpCode::RangeNotSatisfiable
            | HttpCode::ExpectationFailed
            | HttpCode::ImATeapot
            | HttpCode::MisdirectedRequest
            | HttpCode::UnprocessableContent
            | HttpCode::Locked
            | HttpCode::FailedDependency
            | HttpCode::TooEarly
            | HttpCode::UpgradeRequired
            | HttpCode::PreconditionRequired
            | HttpCode::TooManyRequests
            | HttpCode::RequestHeaderFieldsTooLarge
            | HttpCode::UnavailableForLegalReasons => HttpCodeClass::ClientError,

            HttpCode::InternalServerError
            | HttpCode::NotImplemented
            | HttpCode::BadGateway
            | HttpCode::ServiceUnavailable
            | HttpCode::GatewayTimeout
            | HttpCode::HttpVersionNotSupported
            | HttpCode::VariantAlsoNegotiates
            | HttpCode::InsufficientStorage
            | HttpCode::LoopDetected
            | HttpCode::NotExtended
            | HttpCode::NetworkAuthetificationRequired => HttpCodeClass::ServerError,

            HttpCode::None => HttpCodeClass::None,
            HttpCode::Unknown(_) => HttpCodeClass::Unknown,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::HttpCode;

    #[test]
    fn uint_to_code() {
        assert!(matches!(HttpCode::from(307), HttpCode::TemporaryRedirect));
        assert!(matches!(HttpCode::from(410), HttpCode::Gone));
        assert!(matches!(
            HttpCode::from(451),
            HttpCode::UnavailableForLegalReasons
        ));
        assert!(matches!(HttpCode::from(504), HttpCode::GatewayTimeout));
        assert!(matches!(HttpCode::from(999), HttpCode::Unknown(999)));
    }

    #[test]
    fn option_to_code() {
        let none: Option<u32> = None;

        assert!(matches!(HttpCode::from(Some(103)), HttpCode::EarlyHints));
        assert!(matches!(HttpCode::from(Some(200)), HttpCode::Ok));
        assert!(matches!(HttpCode::from(Some(226)), HttpCode::ImUsed));
        assert!(matches!(HttpCode::from(Some(302)), HttpCode::Found));
        assert!(matches!(HttpCode::from(none), HttpCode::None));
    }

    #[test]
    fn code_to_uint() {
        assert_eq!(u32::from(HttpCode::SwitchingProtocols), 101);
        assert_eq!(u32::from(HttpCode::NoContent), 204);
        assert_eq!(u32::from(HttpCode::MultipleChoices), 300);
        assert_eq!(u32::from(HttpCode::NotFound), 404);
        assert_eq!(u32::from(HttpCode::RangeNotSatisfiable), 416);
    }
}
