extern crate actix_web;
extern crate challenge_bypass_ristretto;
use actix_web::{server, App, HttpRequest, Responder};

fn greet(req: &HttpRequest) -> impl Responder {
    format!("Please provide a token")
}

fn return_signed_blinded_token(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("blinded_token").unwrap_or("World");

    let mut rng = OsRng::new().unwrap();
    let server_key = SigningKey::random(&mut rng);
    server_key.sign(&blinded_token).unwrap();
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{blinded_token}", |r| r.f(return_signed_blinded_token))
    }).bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
