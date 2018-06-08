// Rocket.rs doesn't support replace default error template..

use rocket::response::content;
use rocket;

/// formatter
#[inline(always)]
fn fm<I: Into<String>>(code: i32, message: I, description: I) -> content::Json<String> {
    content::Json(format!("{{\"status\":{},\"message\":\"{}\",\"description\":\"{}\"}}", code, message.into(), description.into()))
}

/// routes macro
#[inline(always)]
pub(crate) fn get_routes() -> Vec<rocket::Catcher> {
    return errors![handle_400, handle_401, handle_402, handle_403, handle_404, handle_405, handle_406, handle_407, handle_408, handle_409, handle_410, handle_411, handle_412, handle_413, handle_414, handle_415, handle_416, handle_417, handle_418, handle_421, handle_422, handle_426, handle_428, handle_429, handle_431, handle_451, handle_500, handle_501, handle_503, handle_504, handle_510]
}



#[error(400)]
pub(crate) fn handle_400() -> content::Json<String> {
    fm(400, "Bad Request", "The request could not be understood by the server due to malformed syntax.")
}

#[error(401)]
pub(crate) fn handle_401() -> content::Json<String> {
    fm(401, "Unauthorized", "The request requires user authentication.")
}

#[error(402)]
pub(crate) fn handle_402() -> content::Json<String> {
    fm(402, "Payment Required", "The request could not be processed due to lack of payment.")
}

#[error(403)]
pub(crate) fn handle_403() -> content::Json<String> {
    fm(403, "Forbidden", "The server refused to authorize the request.")
}

#[error(404)]
pub(crate) fn handle_404() -> content::Json<String> {
    fm(404, "Not Found", "The requested resource could not be found.")
}

#[error(405)]
pub(crate) fn handle_405() -> content::Json<String> {
    fm(405, "Method Not Allowed", "The request method is not supported for the requested resource.")
}

#[error(406)]
pub(crate) fn handle_406() -> content::Json<String> {
    fm(406, "Not Acceptable", "The requested resource is capable of generating only content not acceptable according to the Accept headers sent in the request.")
}

#[error(407)]
pub(crate) fn handle_407() -> content::Json<String> {
    fm(407, "Proxy Authentication Required", "Authentication with the proxy is required.")
}

#[error(408)]
pub(crate) fn handle_408() -> content::Json<String> {
    fm(408, "Request Timeout", "The server timed out waiting for the request.")
}

#[error(409)]
pub(crate) fn handle_409() -> content::Json<String> {
    fm(409, "Conflict", "The request could not be processed because of a conflict in the request.")
}

#[error(410)]
pub(crate) fn handle_410() -> content::Json<String> {
    fm(410, "Gone", "The resource requested is no longer available and will not be available again.")
}

#[error(411)]
pub(crate) fn handle_411() -> content::Json<String> {
    fm(411, "Length Required", "The request did not specify the length of its content, which is required by the requested resource.")
}

#[error(412)]
pub(crate) fn handle_412() -> content::Json<String> {
    fm(412, "Precondition Failed", "The server does not meet one of the preconditions specified in the request.")
}

#[error(413)]
pub(crate) fn handle_413() -> content::Json<String> {
    fm(413, "Payload Too Large", "The request is larger than the server is willing or able to process.")
}

#[error(414)]
pub(crate) fn handle_414() -> content::Json<String> {
    fm(414, "URI Too Long", "The URI provided was too long for the server to process.")
}

#[error(415)]
pub(crate) fn handle_415() -> content::Json<String> {
    fm(415, "Unsupported Media Type", "The request entity has a media type which the server or resource does not support.")
}

#[error(416)]
pub(crate) fn handle_416() -> content::Json<String> {
    fm(416, "Range Not Satisfiable", "The portion of the requested file cannot be supplied by the server.")
}

#[error(417)]
pub(crate) fn handle_417() -> content::Json<String> {
    fm(417, "Expectation Failed", "The server cannot meet the requirements of the Expect request-header field.")
}

#[error(418)]
pub(crate) fn handle_418() -> content::Json<String> {
    fm(418, "I'm a teapot", "I was requested to brew coffee, and I am a teapot.")
}

#[error(421)]
pub(crate) fn handle_421() -> content::Json<String> {
    fm(421, "Misdirected Request", "The server cannot produce a response for this request.")
}

#[error(422)]
pub(crate) fn handle_422() -> content::Json<String> {
    fm(422, "Unprocessable Entity", "The request was well-formed but was unable to be followed due to semantic errors.")
}

#[error(426)]
pub(crate) fn handle_426() -> content::Json<String> {
    fm(426, "Upgrade Required", "Switching to the protocol in the Upgrade header field is required.")
}

#[error(428)]
pub(crate) fn handle_428() -> content::Json<String> {
    fm(428, "Precondition Required", "The server requires the request to be conditional.")
}

#[error(429)]
pub(crate) fn handle_429() -> content::Json<String> {
    fm(429, "Too Many Requests", "Too many requests have been received recently.")
}

#[error(431)]
pub(crate) fn handle_431() -> content::Json<String> {
    fm(431, "Request Header Fields Too Large", "The server is unwilling to process the request because either an individual header field, or all the header fields collectively, are too large.")
}

#[error(451)]
pub(crate) fn handle_451() -> content::Json<String> {
    fm(451, "Unavailable For Legal Reasons", "The requested resource is unavailable due to a legal demand to deny access to this resource.")
}

#[error(500)]
pub(crate) fn handle_500() -> content::Json<String> {
    fm(500, "Internal Server Error", "The server encountered an internal error while processing this request.")
}

#[error(501)]
pub(crate) fn handle_501() -> content::Json<String> {
    fm(501, "Not Implemented", "The server either does not recognize the request method, or it lacks the ability to fulfill the request.")
}

#[error(503)]
pub(crate) fn handle_503() -> content::Json<String> {
    fm(503, "Service Unavailable", "The server is currently unavailable.")
}

#[error(504)]
pub(crate) fn handle_504() -> content::Json<String> {
    fm(504, "Gateway Timeout", "The server did not receive a timely response from an upstream server.")
}

#[error(510)]
pub(crate) fn handle_510() -> content::Json<String> {
    fm(510, "Not Extended", "Further extensions to the request are required for the server to fulfill it.")
}