use std::env;

fn main() {
    let pwd = env::current_dir();
    if let Err(err) = pwd {
        println!("{}", err);
        return;
    }

    let pwd = pwd.unwrap();
    codecount::start(pwd);
}
