use crate::traits::controller_trait::Controller;
use axum::{routing::get, routing::post, Json, Router};
use serde_json::{json, Value};

use crate::services::user_service::UserService;

pub struct SimpleController {
    path: String,
}

impl SimpleController {
    async fn json() -> Json<Value> {
        Json(json!({ "data": "Hello, World!" }))
    }

    async fn html() -> String {
        String::from("Hello, World!")
    }

    async fn do_something(body: Json<Value>) {
        let entity_service = UserService::new();
        entity_service.do_something(body.get("message").unwrap().as_str().unwrap().to_string());
    }
}

impl Controller for SimpleController {
    fn new(path: String) -> Self {
        SimpleController { path }
    }

    fn routes(&self) -> Router {
        Router::new()
            .route(&(self.path.clone() + "/json"), get(Self::json))
            .route(&(self.path.clone() + "/html"), get(Self::html))
            .route(
                &(self.path.clone() + "/do-something"),
                post(Self::do_something),
            )
    }
}
