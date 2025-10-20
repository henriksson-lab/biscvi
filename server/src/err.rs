
use actix_error::AsApiError;
use actix_error::AsApiErrorTrait;
use actix_error::ApiError;


#[derive(Debug, AsApiError)]
pub enum MyError {
    //#[api_error(status = "NotFound", msg = "The requested resource was not found.")]
    //NotFound,

    //#[api_error(code = 401, msg = "Authentication required.")]
    //Unauthorized,

    #[api_error(code = 500, msg = "An internal server error occurred.")]
    InternalError,

    #[api_error(code = 500, msg = "An internal server error occurred. {err}")]
    IoError {err: std::io::Error},

    #[api_error(code = 500, msg = "An internal server error occurred. {err}")]
    AnyhowError {err: anyhow::Error},

    #[api_error(code = 500, msg = "An internal server error occurred. {err}")]
    SerdeCborError {err: serde_cbor::Error},

    // This variant's single field of type Option<Value> will be used for `ApiError.details`
    //#[api_error(status = "BadRequest", msg = "Invalid input provided.")]
    //WithDetails(Option<Value>),

    //#[api_error(status = "UnprocessableEntity", msg = "Validation failed for field {field_name}. Issue: {issue}")]
    //ValidationError { field_name: String, issue: String },

    // This variant's single field of type Value will be used for `ApiError.details`
    // No msg is provided, so if Display is not implemented manually, it might be empty or a default.
    // However, the details field will be populated.
    //#[api_error(status = "PaymentRequired")]
    //PaymentData(Value),

    //#[api_error(code = 400, msg = "User ID {0} is invalid.")]
    //InvalidUserId(u32),

    // Example with ignore: field_to_ignore won't be in the auto-generated message if msg wasn't specified.
    // If msg is specified like below, ignore on the field itself is not needed for message generation.
    //#[api_error(status = "BadRequest", msg = "Configuration error with setting: {setting_key}")]
    //ConfigError { setting_key: String, #[api_error(ignore)] _internal_code: u32 },
}



impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::IoError {err: error}
    }
}


impl From<anyhow::Error> for MyError {
    fn from(error: anyhow::Error) -> Self {
        MyError::AnyhowError {err: error}
    }
}


impl From<serde_cbor::Error> for MyError {
    fn from(error: serde_cbor::Error) -> Self {
        MyError::SerdeCborError {err: error}
    }
}
