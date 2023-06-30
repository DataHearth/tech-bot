use std::sync::OnceLock;

use tera::Tera;

use crate::database::DB;

pub static TMPL: OnceLock<Tera> = OnceLock::new();
pub static DB: OnceLock<DB> = OnceLock::new();

pub struct AppState<'a> {
    pub tmpl: &'a Tera,
    pub db: &'a DB,
}
