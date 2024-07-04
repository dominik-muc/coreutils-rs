use cat::Config;
use std::{env, process};

fn main(){
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    if let Err(e) = cat::run(config){
        eprintln!("error: {}", e);
        process::exit(1)
    }
}
