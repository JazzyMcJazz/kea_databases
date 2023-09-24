use std::env;

mod server;
mod website;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    server::main();
}
