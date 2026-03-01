use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Responder, Response},
};
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::okapi::schemars::{self, Map};
use rocket_okapi::{gen::OpenApiGenerator, response::OpenApiResponderInner, OpenApiError};

/// Error messages returned to user
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct PulsarrError {
    /// The title of the error message
    pub err: String,
    /// The description of the error
    pub msg: Option<String>,
    // HTTP Status Code returned
    #[serde(skip)]
    pub http_status_code: u16,
}

impl PulsarrError {
    pub fn validation_error<T: ToString>(message: T) -> Self {
        Self {
            err: "validation error".to_string(),
            msg: Some(message.to_string()),
            http_status_code: 400,
        }
    }
    
    pub fn missing_data(missing_data: String) -> Self {
        Self {
            err: "missing data".to_string(),
            msg: Some(missing_data + " not found but required"),
            http_status_code: 400,
        }
    }

    pub fn forbidden<T: ToString>(message: T) -> Self {
        Self {
            err: "forbidden".to_string(),
            msg: Some(message.to_string()),
            http_status_code: 403,
        }
    }

    pub fn internal_error<T: std::fmt::Display>(context: &str, error: T) -> Self {
        eprintln!("[ERROR] {}: {}", context, error);
        Self {
            err: "internal error".to_string(),
            msg: Some("An unexpected error occurred".to_string()),
            http_status_code: 500,
        }
    }
}


impl OpenApiResponderInner for PulsarrError {
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "400".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [400 Bad Request](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)\n\
                The request given is wrongly formatted or data asked could not be fulfilled. \
                "
                    .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "404".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [404 Not Found](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)\n\
                This response is given when you request a page that does not exists.\
                "
                    .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "403".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [403 Forbidden](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/403)\n\
                You do not have permission to perform this action. \
                "
                    .to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "422".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [422 Unprocessable Entity](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/422)\n\
                This response is given when you request body is not correctly formatted. \
                ".to_string(),
                ..Default::default()
            }),
        );
        responses.insert(
            "500".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [500 Internal Server Error](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)\n\
                This response is given when something wend wrong on the server. \
                ".to_string(),
                ..Default::default()
            }),
        );
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}

impl std::fmt::Display for PulsarrError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Error `{}`: {}",
            self.err,
            self.msg.as_deref().unwrap_or("<no message>")
        )
    }
}

impl std::error::Error for PulsarrError {}

impl<'r> Responder<'r, 'static> for PulsarrError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(ContentType::JSON)
            .status(Status::new(self.http_status_code))
            .ok()
    }
}

impl From<rocket::serde::json::Error<'_>> for PulsarrError {
    fn from(err: rocket::serde::json::Error) -> Self {
        use rocket::serde::json::Error::*;
        match err {
            Io(io_error) => PulsarrError {
                err: "IO Error".to_owned(),
                msg: Some(io_error.to_string()),
                http_status_code: 422,
            },
            Parse(_raw_data, parse_error) => PulsarrError {
                err: "Parse Error".to_owned(),
                msg: Some(parse_error.to_string()),
                http_status_code: 422,
            },
        }
    }
}