use axum::Router;

pub trait Controller {
    fn new(path: String) -> Self;
    fn routes(&self) -> Router;
}
