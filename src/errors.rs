use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Not Found")]
    NotFound,

    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Error while creating JWT token")]
    JWTTokenCreationError,

    #[display(fmt = "Forbidden")]
    Forbidden,

    #[display(fmt = "Invalid Token")]
    InvalidToken,

    #[display(fmt = "Invalid Issuer")]
    InvalidIssuer,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::NotFound => HttpResponse::NotFound().json("Not Found"),
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::Forbidden => HttpResponse::Forbidden().json("Forbidden"),
            ServiceError::InvalidToken => HttpResponse::Unauthorized().json("Invalid Token"),
            ServiceError::InvalidIssuer => HttpResponse::Unauthorized().json("Invalid Issuer"),
            ServiceError::JWTTokenCreationError => {
                HttpResponse::InternalServerError().json("JWTTOKENCREATEERROR")
            }
        }
    }
}

// impl From<diesel::result::Error> for ServiceError {
//     fn from(error: diesel::result::Error) -> ServiceError {
//         match error {
//             diesel::result::Error::NotFound => HttpResponse::NotFound().body("Not Found"),
//             DBError::InvalidCString(_) => todo!(),
//             DBError::DatabaseError(_, _) => todo!(),
//             DBError::NotFound => todo!(),
//             DBError::QueryBuilderError(_) => todo!(),
//             DBError::DeserializationError(_) => todo!(),
//             DBError::SerializationError(_) => todo!(),
//             DBError::RollbackTransaction => todo!(),
//             DBError::AlreadyInTransaction => todo!(),
//             _ => todo!(),
//         }
//     }
// }

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            DBError::NotFound => ServiceError::NotFound,
            _ => ServiceError::InternalServerError,
        }
    }
}
