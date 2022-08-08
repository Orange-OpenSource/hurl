use crate::http::HttpError;
use crate::runner;
use crate::runner::RunnerError;
use hurl_core::ast::SourceInfo;
use hurl_core::error::Error;

/// Textual Output for runner errors
impl Error for runner::Error {
    fn source_info(&self) -> SourceInfo {
        self.clone().source_info
    }

    fn description(&self) -> String {
        match &self.inner {
            RunnerError::InvalidUrl(..) => "Invalid URL".to_string(),
            RunnerError::TemplateVariableNotDefined { .. } => "Undefined variable".to_string(),
            RunnerError::VariableNotDefined { .. } => "Undefined variable".to_string(),
            RunnerError::HttpConnection { .. } => "HTTP connection".to_string(),
            RunnerError::CouldNotResolveProxyName => "HTTP connection".to_string(),
            RunnerError::CouldNotResolveHost(_) => "HTTP connection".to_string(),
            RunnerError::FailToConnect => "HTTP connection".to_string(),
            RunnerError::Timeout => "HTTP connection".to_string(),
            RunnerError::TooManyRedirect => "HTTP connection".to_string(),
            RunnerError::CouldNotParseResponse => "HTTP connection".to_string(),
            RunnerError::SslCertificate { .. } => "SSL certificate".to_string(),
            RunnerError::PredicateValue { .. } => "Assert - predicate value failed".to_string(),
            RunnerError::InvalidRegex {} => "Invalid regex".to_string(),
            RunnerError::FileReadAccess { .. } => "File read access".to_string(),
            RunnerError::QueryInvalidXml { .. } => "Invalid XML".to_string(),
            RunnerError::QueryInvalidXpathEval {} => "Invalid XPath expression".to_string(),
            RunnerError::QueryHeaderNotFound {} => "Header not found".to_string(),
            RunnerError::QueryCookieNotFound {} => "Cookie not found".to_string(),
            RunnerError::AssertHeaderValueError { .. } => "Assert header value".to_string(),
            RunnerError::AssertBodyValueError { .. } => "Assert body value".to_string(),
            RunnerError::AssertVersion { .. } => "Assert HTTP version".to_string(),
            RunnerError::AssertStatus { .. } => "Assert status code".to_string(),
            RunnerError::QueryInvalidJson { .. } => "Invalid JSON".to_string(),
            RunnerError::QueryInvalidJsonpathExpression { .. } => "Invalid JSONPath".to_string(),
            RunnerError::PredicateType { .. } => "Assert - inconsistent predicate type".to_string(),
            RunnerError::SubqueryInvalidInput { .. } => "Subquery error".to_string(),
            RunnerError::InvalidDecoding { .. } => "Invalid decoding".to_string(),
            RunnerError::InvalidCharset { .. } => "Invalid charset".to_string(),
            RunnerError::AssertFailure { .. } => "Assert failure".to_string(),
            RunnerError::UnrenderableVariable { .. } => "Unrenderable variable".to_string(),
            RunnerError::NoQueryResult { .. } => "No query result".to_string(),
            RunnerError::UnsupportedContentEncoding(..) => "Decompression error".to_string(),
            RunnerError::CouldNotUncompressResponse(..) => "Decompression error".to_string(),
            RunnerError::InvalidJson { .. } => "Invalid JSON".to_string(),
            RunnerError::UnauthorizedFileAccess { .. } => "Unauthorized file access".to_string(),
        }
    }

    fn fixme(&self) -> String {
        match &self.inner {
            RunnerError::InvalidUrl(url) => format!("invalid URL <{}>", url),
            RunnerError::TemplateVariableNotDefined { name } => {
                format!("you must set the variable {}", name)
            }
            RunnerError::HttpConnection { message, .. } => message.to_string(),
            RunnerError::CouldNotResolveProxyName => "could not resolve proxy name".to_string(),
            RunnerError::CouldNotResolveHost(host) => format!("could not resolve host <{}>", host),
            RunnerError::FailToConnect => "fail to connect".to_string(),
            RunnerError::Timeout => "timeout has been reached".to_string(),
            RunnerError::TooManyRedirect => "too many redirect".to_string(),
            RunnerError::CouldNotParseResponse => "could not parse response".to_string(),
            RunnerError::SslCertificate(description) => description.clone(),
            RunnerError::AssertVersion { actual, .. } => format!("actual value is <{}>", actual),
            RunnerError::AssertStatus { actual, .. } => format!("actual value is <{}>", actual),
            RunnerError::PredicateValue(value) => {
                format!("actual value is <{}>", value)
            }
            RunnerError::InvalidRegex {} => "regex expression is not valid".to_string(),
            RunnerError::FileReadAccess { value } => format!("file {} can not be read", value),
            RunnerError::QueryInvalidXml { .. } => {
                "the HTTP response is not a valid XML".to_string()
            }
            RunnerError::QueryHeaderNotFound {} => {
                "this header has not been found in the response".to_string()
            }
            RunnerError::QueryCookieNotFound {} => {
                "this cookie has not been found in the response".to_string()
            }
            RunnerError::QueryInvalidXpathEval {} => {
                "the XPath expression is not valid".to_string()
            }
            RunnerError::AssertHeaderValueError { actual } => {
                format!("actual value is <{}>", actual)
            }
            RunnerError::AssertBodyValueError { actual, .. } => {
                format!("actual value is <{}>", actual)
            }
            RunnerError::QueryInvalidJson { .. } => {
                "the HTTP response is not a valid JSON".to_string()
            }
            RunnerError::QueryInvalidJsonpathExpression { value } => {
                format!("the JSONPath expression '{}' is not valid", value)
            }
            RunnerError::PredicateType { .. } => {
                "predicate type inconsistent with value return by query".to_string()
            }
            RunnerError::SubqueryInvalidInput(t) => {
                format!("type <{}> from query result and subquery do not match", t)
            }
            RunnerError::InvalidDecoding { charset } => {
                format!("the body can not be decoded with charset '{}'", charset)
            }
            RunnerError::InvalidCharset { charset } => {
                format!("the charset '{}' is not valid", charset)
            }
            RunnerError::AssertFailure {
                actual,
                expected,
                type_mismatch,
                ..
            } => {
                let additional = if *type_mismatch {
                    "\n>>> types between actual and expected are not consistent"
                } else {
                    ""
                };
                format!("actual:   {}\nexpected: {}{}", actual, expected, additional)
            }
            RunnerError::VariableNotDefined { name } => {
                format!("You must set the variable {}", name)
            }
            RunnerError::UnrenderableVariable { value } => {
                format!("value {} can not be rendered", value)
            }
            RunnerError::NoQueryResult { .. } => "The query didn't return any result".to_string(),
            RunnerError::UnsupportedContentEncoding(algorithm) => {
                format!("compression {} is not supported", algorithm)
            }
            RunnerError::CouldNotUncompressResponse(algorithm) => {
                format!("could not uncompress response with {}", algorithm)
            }
            RunnerError::InvalidJson { value } => {
                format!("actual value is <{}>", value)
            }
            RunnerError::UnauthorizedFileAccess { path } => {
                format!(
                    "unauthorized access to file {}, check --file-root option",
                    path.to_str().unwrap()
                )
            }
        }
    }
}

impl From<HttpError> for RunnerError {
    /// Converts a HttpError to a RunnerError.
    fn from(item: HttpError) -> Self {
        match item {
            HttpError::CouldNotParseResponse => RunnerError::CouldNotParseResponse,
            HttpError::CouldNotUncompressResponse { description } => {
                RunnerError::CouldNotUncompressResponse(description)
            }
            HttpError::InvalidCharset { charset } => RunnerError::InvalidCharset { charset },
            HttpError::InvalidDecoding { charset } => RunnerError::InvalidDecoding { charset },
            HttpError::Libcurl {
                code,
                description,
                url,
            } => RunnerError::HttpConnection {
                message: format!("({}) {}", code, description),
                url,
            },
            HttpError::StatuslineIsMissing { url } => RunnerError::HttpConnection {
                message: "status line is missing".to_string(),
                url,
            },
            HttpError::TooManyRedirect => RunnerError::TooManyRedirect,
            HttpError::UnsupportedContentEncoding { description } => {
                RunnerError::UnsupportedContentEncoding(description)
            }
        }
    }
}
