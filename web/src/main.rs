use actix_web::{web::Data, App, HttpServer};
use routes::index;
use state::DB;
use tera::Tera;

use crate::database::DB as Database;
use crate::state::{AppState, TMPL};

mod database;
mod errors;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                tmpl: TMPL.get_or_init(|| {
                    Tera::new("templates/**/*.html").expect("failed to compile templates")
                }),
                db: DB.get_or_init(|| Database::new()),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
