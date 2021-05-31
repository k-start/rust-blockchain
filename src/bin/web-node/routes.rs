use super::Chain;
use crate::handlers;
use std::convert::Infallible;
use warp::{self, Filter};

fn base_route(
    chain: Chain,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get-block")
        .and(warp::get())
        .and(with_chain(chain))
        .and_then(handlers::get_block)
}

pub fn routes(
    chain: Chain,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    base_route(chain)
}

fn with_chain(chain: Chain) -> impl Filter<Extract = (Chain,), Error = Infallible> + Clone {
    warp::any().map(move || chain.clone())
}
