use std::error::Error;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use tera::Error as TeraError;
use thiserror::Error;

use crate::state::TMPL;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Requested file was not found")]
    NotFound,
    #[error("You are forbidden to access requested file")]
    Forbidden,
    #[error("Internal server error")]
    Internal(String),
}

impl ResponseError for Errors {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let tera = TMPL.get().expect("tera should not be empty");

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(tera.render("error", todo!()).unwrap())
    }
}

pub fn map_tera_err(e: TeraError) -> Errors {
    Errors::Internal(e.to_string())
}
