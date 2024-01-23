use axum::Router;

mod controllers;
mod infra;
mod services;
mod traits;

use infra::database;
use mysql::prelude::*;

use controllers::simple_controller::SimpleController;
use traits::controller_trait::Controller;

#[tokio::main]
async fn main() {
    let pool = database::establish_connection();
    let mut conn = pool.get_conn().unwrap();

    conn.exec_batch(
        r"INSERT INTO user (name) VALUES (:name)",
        vec![("name", "Steven")],
    )
    .unwrap();

    let simple_controller = SimpleController::new(String::from("/simple"));
    let app = Router::new().merge(simple_controller.routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
