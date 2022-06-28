use actix_web::{get, web, HttpResponse};

use crate::authentication::UserId;

#[get("/newsletters")]
pub async fn newsletter_form(
    _user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
