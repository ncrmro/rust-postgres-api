extern crate fake;
extern crate planet_express;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let query = &args[0];
    // let filename = &args[2];

    println!("Searching for {}", query);
    // println!("In file {}", filename);

    planet_express::init()
}
