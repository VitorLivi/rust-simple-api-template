use axum::Router;

mod controllers;
mod models;
mod services;
mod traits;

use controllers::simple_controller::SimpleController;
use traits::controller_trait::Controller;

#[tokio::main]
async fn main() {
    let simple_controller = SimpleController::new(String::from("/simple"));

    let app = Router::new().merge(simple_controller.routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
