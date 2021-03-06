use std::sync::Arc;

use db::DB;
use routes::get_routes;
use warp::Filter;

pub mod filters;
pub mod handlers;
pub mod models;
pub mod routes;
mod services;

pub fn init_api(
    db: Arc<DB>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_routes(db)
}
