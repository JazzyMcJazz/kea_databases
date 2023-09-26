use std::env;

mod server;
mod routes;
mod entity;
mod middleware;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    server::main();
}
