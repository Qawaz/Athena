use actix_web::{dev::ServiceRequest, Error};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use super::jwt_data_decode::authorize;

pub async fn http_auth_extract(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let user_roles = authorize(credentials.token()).await;

    match user_roles {
        Ok(roles) => {
            req.attach(roles);
            Ok(req)
        }
        Err(error) => return Err(Error::from(error)),
    }
}
