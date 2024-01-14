use actix_web::Scope;

mod stream;
mod account;
mod extractors;

pub fn routes() -> Scope {
  Scope::new("/api")
    .service(account::routes())
    .service(stream::stream)
}