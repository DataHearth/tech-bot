use actix_web::{get, web::Data, Result};
use tera::Context;

use crate::errors::map_tera_err;
use crate::state::AppState;

#[get("/")]
async fn index(data: Data<AppState<'_>>) -> Result<String> {
    Ok(data
        .tmpl
        .render("base", &Context::new())
        .map_err(map_tera_err)?)
}
