use super::Chain;
use crate::handlers;
use std::convert::Infallible;
use warp::{self, Filter};

fn get_block(
    chain: Chain,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get-block")
        .and(warp::get())
        .and(with_chain(chain))
        .and_then(handlers::get_block)
}

fn get_balance(
    chain: Chain,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("get-balance" / String)
        .and(warp::get())
        .and(with_chain(chain))
        .and_then(handlers::get_balance)
}

fn mine(chain: Chain) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("mine" / String)
        .and(warp::get())
        .and(with_chain(chain))
        .and_then(handlers::mine)
}

pub fn routes(
    chain: Chain,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_block(chain.clone())
        .or(get_balance(chain.clone()))
        .or(mine(chain))
}

fn with_chain(chain: Chain) -> impl Filter<Extract = (Chain,), Error = Infallible> + Clone {
    warp::any().map(move || chain.clone())
}
