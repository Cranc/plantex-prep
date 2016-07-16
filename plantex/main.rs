extern crate base;
extern crate client;
extern crate server;

fn main() {
    println!("Hi from 'plantex'!");
    let res = client::start_game(client::Config::default());

    if res.is_err() {
        std::process::exit(1);
    }
}
