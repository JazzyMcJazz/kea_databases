use std::env;

mod middleware;
mod server;
mod routes;
mod entity;
mod utils;
mod repo;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    server::main();
}
