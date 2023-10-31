use std::env;

mod entity;
mod middleware;
mod repo;
mod routes;
mod server;
mod utils;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    server::main();
}
